/*
 * Copyright 2022 VMware, Inc.
 * SPDX-License-Identifier: MIT
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

package org.dbsp.sqlCompiler.compiler;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.apache.calcite.runtime.CalciteContextException;
import org.apache.calcite.schema.Schema;
import org.apache.calcite.sql.SqlFunction;
import org.apache.calcite.sql.SqlKind;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.SqlNodeList;
import org.apache.calcite.sql.SqlOperatorTable;
import org.apache.calcite.sql.parser.SqlParseException;
import org.apache.calcite.sql.parser.SqlParserPos;
import org.apache.calcite.sql.util.SqlOperatorTables;
import org.dbsp.sqlCompiler.circuit.DBSPCircuit;
import org.dbsp.sqlCompiler.circuit.DBSPPartialCircuit;
import org.dbsp.sqlCompiler.circuit.operator.DBSPSourceTableOperator;
import org.dbsp.sqlCompiler.compiler.backend.ToDotVisitor;
import org.dbsp.sqlCompiler.compiler.errors.BaseCompilerException;
import org.dbsp.sqlCompiler.compiler.errors.CompilationError;
import org.dbsp.sqlCompiler.compiler.errors.CompilerMessages;
import org.dbsp.sqlCompiler.compiler.errors.SourceFileContents;
import org.dbsp.sqlCompiler.compiler.errors.SourcePositionRange;
import org.dbsp.sqlCompiler.compiler.errors.UnsupportedException;
import org.dbsp.sqlCompiler.compiler.frontend.TypeCompiler;
import org.dbsp.sqlCompiler.compiler.frontend.calciteCompiler.ForeignKey;
import org.dbsp.sqlCompiler.compiler.frontend.calciteObject.CalciteObject;
import org.dbsp.sqlCompiler.compiler.frontend.CalciteToDBSPCompiler;
import org.dbsp.sqlCompiler.compiler.frontend.TableContents;
import org.dbsp.sqlCompiler.compiler.frontend.calciteCompiler.CalciteCompiler;
import org.dbsp.sqlCompiler.compiler.frontend.calciteCompiler.CustomFunctions;
import org.dbsp.sqlCompiler.compiler.frontend.parser.SqlFragment;
import org.dbsp.sqlCompiler.compiler.frontend.statements.CreateFunctionStatement;
import org.dbsp.sqlCompiler.compiler.frontend.statements.CreateTableStatement;
import org.dbsp.sqlCompiler.compiler.frontend.statements.CreateViewStatement;
import org.dbsp.sqlCompiler.compiler.frontend.statements.FrontEndStatement;
import org.dbsp.sqlCompiler.compiler.frontend.statements.IHasSchema;
import org.dbsp.sqlCompiler.compiler.frontend.parser.SqlLateness;
import org.dbsp.sqlCompiler.compiler.frontend.statements.LatenessStatement;
import org.dbsp.sqlCompiler.compiler.visitors.outer.CircuitOptimizer;
import org.dbsp.sqlCompiler.ir.DBSPFunction;
import org.dbsp.sqlCompiler.ir.DBSPNode;
import org.dbsp.sqlCompiler.ir.expression.DBSPVariablePath;
import org.dbsp.sqlCompiler.ir.type.DBSPTypeCode;
import org.dbsp.sqlCompiler.ir.type.derived.DBSPTypeStruct;
import org.dbsp.sqlCompiler.ir.type.user.DBSPTypeUser;
import org.dbsp.util.IWritesLogs;
import org.dbsp.util.Linq;
import org.dbsp.util.Logger;
import org.dbsp.util.Utilities;

import javax.annotation.Nullable;
import java.io.IOException;
import java.io.InputStream;
import java.io.PrintStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * This class compiles SQL statements into DBSP circuits.
 * The protocol is:
 * - create compiler
 * - repeat as much as necessary:
 *   - compile a sequence of SQL statements
 *     (CREATE TABLE, CREATE VIEW)
 *   - get the resulting circuit, starting a new one
 * This protocol allows one compiler to generate multiple independent circuits.
 * The compiler can also compile INSERT statements by simulating their
 * execution and keeping track of the contents of each table.
 * The contents after insertions can be obtained using getTableContents().
 */
public class DBSPCompiler implements IWritesLogs, ICompilerComponent, IErrorReporter {
    /** Name of the Rust file that will contain the user-defined functions.
     * The definitions supplied by the user will be copied here. */
    public static final String UDF_FILE_NAME = "udf.rs";
    /** Name of the rust file that will contain stubs for all user-defined functions */
    public static final String STUBS_FILE_NAME = "stubs.rs";

    final GlobalTypes globalTypes = new GlobalTypes();

    /** Convert the string to the casing specified by the options */
    public String toCase(String name) {
        return this.options.canonicalName(name);
    }

    /** Where does the compiled program come from? */
    public enum InputSource {
        /** No data source set yet. */
        None,
        /** Data received from stdin. */
        Stdin,
        /** Data read from a file.  We read the entire file upfront, and then we compile. */
        File,
        /** Data received through API calls (compileStatement/s). */
        API,
    }

    /** Variable that refers to the weight of the row in the z-set. */
    public final DBSPVariablePath weightVar;

    public final CalciteCompiler frontend;
    final CalciteToDBSPCompiler midend;
    public final CompilerOptions options;
    public final CompilerMessages messages;
    public final SourceFileContents sources;
    public InputSource inputSources = InputSource.None;
    public final ProgramMetadata metadata;

    public final TypeCompiler typeCompiler;
    public boolean hasWarnings;

    final Map<String, CreateViewStatement> views = new HashMap<>();
    final List<LatenessStatement> lateness = new ArrayList<>();
    /** All UDFs from the SQL program.  The ones in Rust have no bodies */
    public final List<DBSPFunction> functions = new ArrayList<>();

    /** Circuit produced by the compiler. */
    @Nullable DBSPCircuit circuit;

    public DBSPCompiler(CompilerOptions options) {
        this.options = options;
        // Setting these first allows errors to be reported
        this.messages = new CompilerMessages(this);
        this.metadata = new ProgramMetadata();
        this.frontend = new CalciteCompiler(options, this);
        this.midend = new CalciteToDBSPCompiler(true, options,
                this, this.metadata);
        this.sources = new SourceFileContents();
        this.circuit = null;
        this.typeCompiler = new TypeCompiler(this);
        this.weightVar = new DBSPTypeUser(CalciteObject.EMPTY, DBSPTypeCode.USER, "Weight", false)
                .var();
        this.start();
    }

    // Steps executed before the actual compilation.
    void start() {
        // Declare the "NOW" table
        this.compileInternal(
                "CREATE TABLE NOW(now TIMESTAMP NOT NULL LATENESS INTERVAL 0 SECONDS)",
                false, false);
    }

    public boolean hasWarnings() {
        return this.hasWarnings;
    }

    public CustomFunctions getCustomFunctions() {
        return this.frontend.getCustomFunctions();
    }

    @Override
    public DBSPCompiler compiler() {
        return this;
    }

    public TypeCompiler getTypeCompiler() {
        return this.typeCompiler;
    }

    public String getSaneStructName(String name) {
        return this.globalTypes.generateSaneName(name);
    }

    public void registerStruct(DBSPTypeStruct type) {
        this.globalTypes.register(type);
    }

    @Nullable
    public DBSPTypeStruct getStructByName(String name) {
        return this.globalTypes.getStructByName(name);
    }

    public boolean isStructConstructor(String name) {
        return this.globalTypes.containsStruct(name);
    }

    /**
     * Report an error or warning during compilation.
     * @param range      Position in source where error is located.
     * @param warning    True if this is a warning.
     * @param errorType  A short string that categorizes the error type.
     * @param message    Error message.
     */
    public void reportProblem(SourcePositionRange range, boolean warning,
                              String errorType, String message) {
        if (warning)
            this.hasWarnings = true;
        this.messages.reportProblem(range, warning, errorType, message);
        if (!warning && this.options.languageOptions.throwOnError) {
            System.err.println(this.messages);
            throw new CompilationError("Error during compilation");
        }
    }

    void setSource(InputSource source) {
        if (this.inputSources != InputSource.None &&
                this.inputSources != source)
            throw new UnsupportedException("Input data already received from " + this.inputSources,
                    CalciteObject.EMPTY);
        this.inputSources = source;
    }

    /** Add a new source which can provide schema information about input tables */
    public void addSchemaSource(String name, Schema schema) {
        this.frontend.addSchemaSource(name, schema);
    }

    record SqlStatements(String statement, boolean many, boolean visible) {}

    final List<SqlStatements> toCompile = new ArrayList<>();

    /** Accumulate program to compile.
     * @param statements Statements to compile.
     * @param many       Is this one or many statements?
     * @param appendToSource  If true this is part of the source supplied by the user.
     *                        Otherwise, it's a "preamble" generated internally.
     */
    private void compileInternal(String statements, boolean many, boolean appendToSource) {
        if (this.inputSources != InputSource.File) {
            // If we read from file we already have read the entire data.
            // Otherwise, we append the statements to the sources.
            if (appendToSource) {
                this.sources.append(statements);
                this.sources.append("\n");
            }
        }
        this.toCompile.add(new SqlStatements(statements, many, appendToSource));
    }

    private void compileInternal(String statements, boolean many) {
        this.compileInternal(statements, many, true);
    }

    void validateForeignKeys(DBSPCircuit circuit, List<ForeignKey> foreignKeys) {
        // Invoked after all statements have been compiled and thus
        // all tables in the program are known.
        for (ForeignKey fk: foreignKeys) {
            ForeignKey.TableAndColumns self = fk.thisTable;
            ForeignKey.TableAndColumns other = fk.otherTable;
            String thisTableName = self.tableName.getString();

            if (self.columns.size() != other.columns.size()) {
                this.reportError(self.listPos, "Size mismatch",
                        "FOREIGN KEY section of table " +
                                Utilities.singleQuote(thisTableName) +
                                " contains " + self.columns.size() + " columns," +
                                " which does not match the size the REFERENCES, which is " +
                                other.columns.size());
                continue;
            }

            // Check that the referred columns exist and have proper types
            String otherTableName = other.tableName.getString();
            DBSPSourceTableOperator otherTable = circuit.getInput(otherTableName);
            if (otherTable == null) {
                this.reportWarning(other.tableName.getSourcePosition(),
                        "Table not found",
                        "Table " + Utilities.singleQuote(otherTableName)
                                + ", referred in FOREIGN KEY constraint of table " +
                                Utilities.singleQuote(thisTableName) + ", does not exist");
                continue;
            }

            DBSPSourceTableOperator thisTable = circuit.getInput(thisTableName);
            assert thisTable != null;

            List<InputColumnMetadata> otherKeys = otherTable.metadata.getPrimaryKeys();
            if (otherKeys.size() != self.columns.size()) {
                this.reportError(self.listPos,
                        "PRIMARY KEY does not match",
                        "The PRIMARY KEY of table " + Utilities.singleQuote(otherTableName) +
                                " does not match the FOREIGN KEY of " + Utilities.singleQuote(thisTableName));
                continue;
            }

            for (int i = 0; i < self.columns.size(); i++) {
                SqlFragment selfColumn = self.columns.get(i);
                SqlFragment otherColumn = other.columns.get(i);
                String selfColumnName = selfColumn.getString();
                String otherColumnName = otherColumn.getString();
                InputColumnMetadata selfMeta = thisTable.metadata.getColumnMetadata(selfColumnName);
                if (selfMeta == null) {
                    this.reportError(selfColumn.getSourcePosition(),
                            "Column not found",
                            "Table " + Utilities.singleQuote(thisTableName) +
                                    " does not have a column named " + Utilities.singleQuote(selfColumnName));
                    continue;
                }
                InputColumnMetadata otherMeta = otherTable.metadata.getColumnMetadata(otherColumnName);
                if (otherMeta == null) {
                    this.reportError(selfColumn.getSourcePosition(),
                            "Column not found",
                            "Table " + Utilities.singleQuote(otherTableName) +
                                    " does not have a column named " + Utilities.singleQuote(otherColumnName));
                    continue;
                }
                if (!otherMeta.isPrimaryKey) {
                    this.reportError(selfColumn.getSourcePosition(),
                            "FOREIGN KEY points to non-key column",
                            "FOREIGN KEY column " +
                                    Utilities.singleQuote(thisTableName + "." + selfColumnName) +
                                    " refers to column " +
                                    Utilities.singleQuote(otherTableName + "." + otherColumnName) +
                                    " which is not a PRIMARY KEY");
                    continue;
                }
                if (!selfMeta.type.setMayBeNull(false).sameType(otherMeta.type.setMayBeNull(false))) {
                    this.reportError(selfColumn.getSourcePosition(),
                            "Mismatched FOREIGN KEY column types",
                            "FOREIGN KEY column " +
                                    Utilities.singleQuote(thisTableName + "." + selfColumnName) +
                                    " has type " + selfMeta.type.asSqlString() +
                                    " which does not match the type " + otherMeta.type.asSqlString() +
                                    " of the referenced column " +
                                    Utilities.singleQuote(otherTableName + "." + otherColumnName));
                }
            }
        }
    }

    void printMessages(@Nullable CreateViewStatement statement) {
        System.err.println(this.messages);
        if (statement != null) {
            System.err.println("While compiling");
            System.err.println(this.sources.getFragment(
                    new SourcePositionRange(statement.createView.getParserPosition()), true));
        }
    }

    void runAllCompilerStages() {
        CreateViewStatement currentView = null;
        try {
            // Parse using Calcite
            SqlNodeList parsed = new SqlNodeList(SqlParserPos.ZERO);
            // across all tables
            List<ForeignKey> foreignKeys = new ArrayList<>();

            for (SqlStatements stat: this.toCompile) {
                if (stat.many) {
                    if (stat.statement.isEmpty())
                        continue;
                    parsed.addAll(this.frontend.parseStatements(stat.statement, stat.visible));
                } else {
                    SqlNode node = this.frontend.parse(stat.statement, stat.visible);
                    List<SqlNode> stmtList = new ArrayList<>();
                    stmtList.add(node);
                    parsed.addAll(new SqlNodeList(stmtList, node.getParserPosition()));
                }
                if (this.hasErrors())
                    return;
            }
            this.toCompile.clear();

            // All UDFs which have no bodies in SQL
            final List<SqlFunction> rustFunctions = new ArrayList<>();
            // Compile first the statements that define functions, types, and lateness
            for (SqlNode node: parsed) {
                Logger.INSTANCE.belowLevel(this, 2)
                        .append("Parsing result: ")
                        .append(node.toString())
                        .newline();
                SqlKind kind = node.getKind();
                if (kind == SqlKind.CREATE_TYPE) {
                    FrontEndStatement fe = this.frontend.compile(node.toString(), node, this.sources);
                    if (fe == null)
                        // error during compilation
                        continue;
                    this.midend.compile(fe);
                    continue;
                }
                if (kind == SqlKind.CREATE_FUNCTION) {
                    FrontEndStatement fe = this.frontend.compile(node.toString(), node, this.sources);
                    if (fe == null)
                        continue;
                    CreateFunctionStatement stat = fe.to(CreateFunctionStatement.class);
                    SqlFunction function = stat.function;
                    if (!stat.function.isSqlFunction()) {
                        rustFunctions.add(function);
                    } else {
                        // Reload the operator table to include the newly defined SQL function.
                        // This allows the functions ot be used in other function definitions.
                        // There should be a better way to do this.
                        SqlOperatorTable newFunctions = SqlOperatorTables.of(Linq.list(function));
                        this.frontend.addOperatorTable(newFunctions);
                    }
                    DBSPNode func = this.midend.compile(fe);
                    this.functions.add(Objects.requireNonNull(func).to(DBSPFunction.class));
                }
                if (node instanceof SqlLateness) {
                    FrontEndStatement fe = this.frontend.compile(node.toString(), node, this.sources);
                    if (fe == null)
                        continue;
                    this.lateness.add(fe.to(LatenessStatement.class));
                    this.midend.compile(fe);
                }
            }

            if (!rustFunctions.isEmpty()) {
                // Reload the operator table to include the newly defined Rust function.
                // These we can load all at the end, since they can't depend on each other.
                SqlOperatorTable newFunctions = SqlOperatorTables.of(rustFunctions);
                this.frontend.addOperatorTable(newFunctions);
            }

            // Compile all statements which do not define functions or types
            for (SqlNode node : parsed) {
                SqlKind kind = node.getKind();
                if (kind == SqlKind.CREATE_FUNCTION || kind == SqlKind.CREATE_TYPE)
                    continue;
                if (node instanceof SqlLateness)
                    continue;
                FrontEndStatement fe = this.frontend.compile(node.toString(), node, this.sources);
                if (fe == null)
                    // error during compilation
                    continue;
                if (fe.is(CreateViewStatement.class)) {
                    CreateViewStatement cv = fe.to(CreateViewStatement.class);
                    currentView = cv;
                    this.views.put(cv.getName(), cv);
                } else if (fe.is(CreateTableStatement.class)) {
                    CreateTableStatement ct = fe.to(CreateTableStatement.class);
                    foreignKeys.addAll(ct.foreignKeys);
                }
                this.midend.compile(fe);
                currentView = null;
            }

            this.circuit = this.midend.getFinalCircuit().seal("parsed");
            this.validateForeignKeys(this.circuit, foreignKeys);
            this.optimize();
        } catch (SqlParseException e) {
            if (e.getCause() instanceof BaseCompilerException) {
                // Exceptions we throw in parser validation code are caught
                // by the Calcite parser and wrapped in SqlParseException.
                // Unwrap them to retrieve source position...
                this.messages.reportError((BaseCompilerException) e.getCause());
            } else {
                this.messages.reportError(e);
            }
            if (this.options.languageOptions.throwOnError) {
                this.printMessages(currentView);
                throw new RuntimeException(e);
            }
        } catch (CalciteContextException e) {
            this.messages.reportError(e);
            if (this.options.languageOptions.throwOnError) {
                this.printMessages(currentView);
                throw new RuntimeException(e);
            }
        } catch (BaseCompilerException e) {
            this.messages.reportError(e);
            if (this.options.languageOptions.throwOnError) {
                this.printMessages(currentView);
                throw e;
            }
        } catch (Throwable e) {
            this.messages.reportError(e);
            if (this.options.languageOptions.throwOnError) {
                this.printMessages(currentView);
                throw e;
            }
        }
    }

    public ObjectNode getIOMetadataAsJson() {
        ObjectMapper mapper = Utilities.deterministicObjectMapper();
        ArrayNode inputs = mapper.createArrayNode();
        for (IHasSchema input: this.metadata.inputTables.values())
            inputs.add(input.asJson());
        ArrayNode outputs = mapper.createArrayNode();
        for (IHasSchema output: this.metadata.outputViews.values())
            outputs.add(output.asJson());
        ObjectNode ios = mapper.createObjectNode();
        ios.set("inputs", inputs);
        ios.set("outputs", outputs);
        return ios;
    }

    public void compileStatements(String program) {
        this.setSource(InputSource.API);
        this.compileInternal(program, true);
    }

    void optimize() {
        if (this.circuit == null) {
            this.circuit = this.midend.getFinalCircuit().seal("tmp");
        }
        CircuitOptimizer optimizer = new CircuitOptimizer(this);
        this.circuit = optimizer.optimize(this.circuit);
    }

    public void removeNowTable() {
        String name = this.toCase("now");
        this.metadata.removeTable(name);
        this.midend.getTableContents().removeTable(name);
    }

    public void compileStatement(String statement) {
        this.compileInternal(statement, false);
    }

    public void setEntireInput(@Nullable String filename, InputStream contents) throws IOException {
        if (filename != null)
            this.setSource(InputSource.File);
        else
            this.setSource(InputSource.Stdin);
        this.sources.setEntireInput(filename, contents);
    }

    public void compileInput() {
        if (this.inputSources == InputSource.None)
            throw new UnsupportedException("compileInput has been called without calling setEntireInput",
                    CalciteObject.EMPTY);
        this.compileInternal(this.sources.getWholeProgram(), true);
    }

    public boolean hasErrors() {
        return this.messages.exitCode != 0;
    }

    /** Perform checks that can only be made after all the statements have been compiled */
    void postCompilationChecks() {
        // Check that all views mentioned in LATENESS statements exist
        for (LatenessStatement late: this.lateness) {
            String view = late.view.getSimple();
            if (!this.views.containsKey(view)) {
                this.compiler().reportWarning(late.getPosition(), "No such view",
                        "No view named " + Utilities.singleQuote(view) + " found");
            }
        }
    }

    /** Run all compilation stages.
     * Get the circuit generated by compiling the statements to far.
     * Start a new circuit.
     * @param name  Name to use for the produced circuit. */
    public DBSPCircuit getFinalCircuit(String name) {
        this.runAllCompilerStages();

        if (this.circuit == null) {
            DBSPPartialCircuit circuit = this.midend.getFinalCircuit();
            this.circuit = circuit.seal(name);
        }

        this.postCompilationChecks();

        DBSPCircuit result = this.circuit.rename(name);
        this.circuit = null;
        if (this.getDebugLevel() > 0 && !result.name.equals("tmp")) {
            ToDotVisitor.toDot(
                    this, "final.png", this.getDebugLevel(), "png", result);
        }
        return result;
    }

    /** Get the contents of the tables as a result of all the INSERT statements compiled. */
    public TableContents getTableContents() {
        this.runAllCompilerStages();
        return this.midend.getTableContents();
    }

    /** Empty the contents of all tables that were populated by INSERT or DELETE statements */
    public void clearTables() {
        this.midend.clearTables();
    }

    public void showErrors(PrintStream stream) {
        this.messages.show(stream);
    }

    /** Throw if any error has been encountered.
     * Displays the errors on stderr as well. */
    public void throwIfErrorsOccurred() {
        if (this.hasErrors()) {
            this.showErrors(System.err);
            throw new CompilationError("Error during compilation");
        }
    }
}
