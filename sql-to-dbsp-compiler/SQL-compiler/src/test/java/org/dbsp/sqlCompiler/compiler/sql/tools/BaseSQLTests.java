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

package org.dbsp.sqlCompiler.compiler.sql.tools;

import org.dbsp.sqlCompiler.circuit.DBSPCircuit;
import org.dbsp.sqlCompiler.compiler.CompilerOptions;
import org.dbsp.sqlCompiler.compiler.DBSPCompiler;
import org.dbsp.sqlCompiler.compiler.backend.rust.RustFileWriter;
import org.dbsp.sqlCompiler.compiler.frontend.TableContents;
import org.dbsp.sqlCompiler.compiler.frontend.calciteCompiler.CalciteCompiler;
import org.dbsp.sqlCompiler.compiler.visitors.outer.LowerCircuitVisitor;
import org.dbsp.sqlCompiler.compiler.visitors.outer.monotonicity.MonotoneAnalyzer;
import org.dbsp.sqlCompiler.ir.type.DBSPType;
import org.dbsp.util.Logger;
import org.dbsp.util.ProgramAndTester;
import org.dbsp.util.Utilities;
import org.junit.AfterClass;
import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Rule;
import org.junit.rules.TestName;
import org.junit.rules.TestWatcher;
import org.junit.runner.Description;

import java.io.File;
import java.io.IOException;
import java.io.PrintStream;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

/** Base class for SQL-based tests. */
public class BaseSQLTests {
    /** Override this method to prepare the tables on
     * which the tests are built. */
    public void prepareInputs(DBSPCompiler compiler) {}

    public CompilerCircuitStream getCCS(String sql) {
        DBSPCompiler compiler = this.testCompiler();
        compiler.compileStatements(sql);
        return new CompilerCircuitStream(compiler);
    }

    @SuppressWarnings("unused")
    protected void showPlan() {
        Logger.INSTANCE.setLoggingLevel(CalciteCompiler.class, 2);
    }

    @SuppressWarnings("unused")
    protected void instrumentApply() {
        Logger.INSTANCE.setLoggingLevel(LowerCircuitVisitor.class, 2);
    }

    @SuppressWarnings("unused")
    protected void showMonotone() {
        Logger.INSTANCE.setLoggingLevel(MonotoneAnalyzer.class, 2);
    }

    @SuppressWarnings("unused")
    protected void showFinal() {
        Logger.INSTANCE.setLoggingLevel(DBSPCompiler.class, 2);
    }

    @SuppressWarnings("unused")
    protected void showFinalVerbose() {
        Logger.INSTANCE.setLoggingLevel(DBSPCompiler.class, 4);
    }

    /** Run a query that is expected to fail in compilation.
     * @param query             Query to run.
     * @param messageFragment   This fragment should appear in the error message. */
    public void queryFailingInCompilation(String query, String messageFragment) {
        this.statementsFailingInCompilation("CREATE VIEW VV AS " + query, messageFragment);
    }

    /** Run one or more statements that are expected to fail in compilation.
     * @param statements        Statements to compile.
     * @param messageFragment   This fragment should appear in the error message. */
    public void statementsFailingInCompilation(String statements, String messageFragment) {
        DBSPCompiler compiler = this.testCompiler();
        compiler.options.languageOptions.throwOnError = false;
        this.prepareInputs(compiler);
        compiler.compileStatements(statements);
        getCircuit(compiler);
        Assert.assertTrue(compiler.messages.exitCode != 0);
        // compiler.options.ioOptions.emitJsonErrors = true;
        String message = compiler.messages.toString();
        // System.out.println(message);
        boolean contains = message.contains(messageFragment);
        if (!contains)
            Assert.fail("Error message\n" + Utilities.singleQuote(message) +
                    "\ndoes not contain the expected fragment\n" + Utilities.singleQuote(messageFragment));
    }

    /** Compile a set of statements that are expected to give a warning at compile time.
     * @param statements        Statement to run.
     * @param messageFragment   This fragment should appear in the warning message. */
    public void shouldWarn(String statements, String messageFragment) {
        DBSPCompiler compiler = this.testCompiler();
        compiler.options.languageOptions.throwOnError = false;
        this.prepareInputs(compiler);
        compiler.compileStatements(statements);
        getCircuit(compiler);
        Assert.assertTrue(compiler.hasWarnings);
        String warnings = compiler.messages.messages.stream()
                .filter(error -> error.warning).toList().toString();
        boolean contains = warnings.contains(messageFragment);
        if (!contains)
            Assert.fail("Error message\n" + Utilities.singleQuote(warnings) +
                    "\ndoes not contain the expected fragment\n" + Utilities.singleQuote(messageFragment));
    }

    /** Helper class for testing.  Holds together
     * - the compiler that is used to compile a program,
     * - the circuit, and
     * - the input/output data that is used to test the circuit. */
    public static class CompilerCircuitStream {
        public final DBSPCompiler compiler;
        public final DBSPCircuit circuit;
        final InputOutputChangeStream stream;

        public CompilerCircuitStream(DBSPCompiler compiler) {
            this(compiler, new InputOutputChangeStream());
        }

        public CompilerCircuitStream(DBSPCompiler compiler, InputOutputChangeStream streams) {
            this.compiler = compiler;
            this.circuit = BaseSQLTests.getCircuit(compiler);
            this.stream = streams;
        }

        public void showErrors() {
            this.compiler.messages.show(System.err);
            this.compiler.messages.clear();
        }

        /** Compiles a SQL script composed of INSERT statements.
         * into a Change. */
        public Change toChange(String script) {
            this.compiler.clearTables();
            this.compiler.compileStatements(script);
            TableContents tableContents = this.compiler.getTableContents();
            return new Change(tableContents);
        }

        /** Add a step to a change stream with many input tables but one single output view.
         * A step is described as an input-output pair.
         * @param script   SQL script that describes insertions and deletions into the input tables.
         * @param expected A text representation of the output produced for this step with an extra last
         *                 column that contains weights.
         */
        public void step(String script, String expected) {
            Change input = this.toChange(script);
            DBSPType outputType = circuit.getSingleOutputType();
            Change output = TableParser.parseChangeTable(expected, outputType);
            stream.addPair(input, output);
        }

        public void addChange(InputOutputChange ioChange) {
            this.stream.addChange(ioChange);
        }

        public void addPair(Change inputChange, Change outputChange) {
            this.stream.addPair(inputChange, outputChange);
        }
    }

    @Rule
    public TestName currentTestName = new TestName();

    String currentTestInformation = "";

    @Rule
    public TestWatcher testWatcher = new TestWatcher() {
        @Override
        protected void starting(final Description description) {
            String methodName = description.getMethodName();
            String className = description.getClassName();
            className = className.substring(className.lastIndexOf('.') + 1);
            BaseSQLTests.this.currentTestInformation  = className + "#" + methodName;
        }
    };

    public static final String projectDirectory = "..";
    public static final String rustDirectory = projectDirectory + "/temp/src";
    public static final String testFilePath = rustDirectory + "/lib.rs";

    public static int testsExecuted = 0;

    /** Collect here all the tests to run and execute them using a single Rust compilation. */
    static final List<TestCase> testsToRun = new ArrayList<>();

    @BeforeClass
    public static void prepareTests() {
        testsToRun.clear();
    }

    public static File createInputFile(File file, String contents) throws IOException {
        file.deleteOnExit();
        PrintWriter script = new PrintWriter(file, StandardCharsets.UTF_8);
        script.println(contents);
        script.close();
        return file;
    }

    public static File createInputScript(String contents) throws IOException {
        File result = File.createTempFile("script", ".sql", new File(rustDirectory));
        return createInputFile(result, contents);
    }

    DBSPCompiler noThrowCompiler() {
        DBSPCompiler compiler = this.testCompiler();
        compiler.options.languageOptions.throwOnError = false;
        return compiler;
    }

    /** Runs all the tests from the testsToRun list. */
    @AfterClass
    public static void runAllTests() throws IOException, InterruptedException {
        if (testsToRun.isEmpty())
            return;
        // Create empty stubs.rs file
        PrintStream outputStream = new PrintStream(Files.newOutputStream(Paths.get(rustDirectory, DBSPCompiler.STUBS_FILE_NAME)));
        outputStream.println();
        outputStream.close();
        outputStream = new PrintStream(Files.newOutputStream(Paths.get(testFilePath)));
        // Use the compiler from the first test case.
        DBSPCompiler firstCompiler = testsToRun.get(0).ccs.compiler;
        RustFileWriter writer = new RustFileWriter(outputStream);
        int testNumber = 0;
        String[] extraArgs = new String[0];
        for (TestCase test: testsToRun) {
            if (!test.ccs.compiler.options.same(firstCompiler.options))
                throw new RuntimeException("Test " + Utilities.singleQuote(testsToRun.get(0).javaTestName) +
                        " and " + Utilities.singleQuote(test.javaTestName) +
                        " are not compiled with the same options: "
                        + test.ccs.compiler.options.diff(firstCompiler.options));
            ProgramAndTester pt;
            test.ccs.circuit.setName("circuit" + testNumber);
            pt = new ProgramAndTester(test.ccs.circuit, test.createTesterCode(testNumber, rustDirectory));
            BaseSQLTests.testsExecuted++;
            writer.add(pt);
            testNumber++;
        }
        writer.writeAndClose(firstCompiler);
        Utilities.compileAndTestRust(rustDirectory, true, extraArgs);
        testsToRun.clear();
    }

    public void addRustTestCase(
            CompilerCircuitStream ccs) {
        ccs.compiler.messages.show(System.err);
        if (ccs.compiler.messages.exitCode != 0)
            throw new RuntimeException("Compilation failed");
        ccs.compiler.messages.clear();
        TestCase test = new TestCase(this.currentTestInformation, this.currentTestInformation, ccs, null);
        testsToRun.add(test);
    }

    /** Add a test case without inputs */
    public void compileRustTestCase(String statements) {
        DBSPCompiler compiler = this.testCompiler();
        compiler.compileStatements(statements);
        Assert.assertFalse(compiler.hasErrors());
        CompilerCircuitStream ccs = new CompilerCircuitStream(compiler);
        this.addRustTestCase(ccs);
    }

    protected void addFailingRustTestCase(
            String name, String message, CompilerCircuitStream ccs) {
        ccs.showErrors();
        TestCase test = new TestCase(name, this.currentTestInformation, ccs, message);
        testsToRun.add(test);
    }

    /**
     * Create CompilerOptions according to the specified properties.
     * @param incremental  Generate an incremental program if true.
     * @param optimize     Optimize program if true.
     */
    public CompilerOptions testOptions(boolean incremental, boolean optimize) {
        CompilerOptions options = new CompilerOptions();
        options.languageOptions.throwOnError = true;
        options.languageOptions.generateInputForEveryTable = true;
        options.ioOptions.quiet = true;
        options.ioOptions.emitHandles = true;
        options.languageOptions.incrementalize = incremental;
        options.languageOptions.unrestrictedIOTypes = true;
        options.languageOptions.optimizationLevel = optimize ? 2 : 1;
        return options;
    }

    /** Return the default compiler used for testing. */
    public DBSPCompiler testCompiler() {
        CompilerOptions options = this.testOptions(false, true);
        return new DBSPCompiler(options);
    }

    public static DBSPCircuit getCircuit(DBSPCompiler compiler) {
        return compiler.getFinalCircuit(false);
    }

    protected InputOutputChangeStream streamWithEmptyChanges() {
        return new InputOutputChangeStream().addChange(
                new InputOutputChange(new Change(), new Change()));
    }

    protected void runtimeFail(String query, String message, InputOutputChangeStream data) {
        query = "CREATE VIEW V AS " + query;
        DBSPCompiler compiler = this.testCompiler();
        compiler.compileStatement(query);
        CompilerCircuitStream ccs = new CompilerCircuitStream(compiler, data);
        this.addFailingRustTestCase(query, message, ccs);
    }

    /** Run a test that fails at runtime without needing any inputs */
    protected void runtimeConstantFail(String query, String message) {
        this.runtimeFail(query, message, this.streamWithEmptyChanges());
    }
}
