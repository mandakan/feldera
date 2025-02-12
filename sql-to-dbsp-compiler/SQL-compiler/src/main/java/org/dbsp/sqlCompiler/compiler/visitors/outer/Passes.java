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

package org.dbsp.sqlCompiler.compiler.visitors.outer;

import org.dbsp.sqlCompiler.circuit.DBSPCircuit;
import org.dbsp.sqlCompiler.compiler.DBSPCompiler;
import org.dbsp.sqlCompiler.compiler.ICompilerComponent;
import org.dbsp.sqlCompiler.compiler.backend.dot.ToDot;
import org.dbsp.sqlCompiler.compiler.visitors.inner.InnerRewriteVisitor;
import org.dbsp.util.IWritesLogs;
import org.dbsp.util.Linq;
import org.dbsp.util.Logger;

import java.util.List;

public class Passes implements IWritesLogs, CircuitTransform, ICompilerComponent {
    final DBSPCompiler compiler;
    public final List<CircuitTransform> passes;
    // Generate a new name for each dumped circuit.
    static int dumped = 0;
    final long id;
    final String name;

    public Passes(String name, DBSPCompiler reporter, CircuitTransform... passes) {
        this(name, reporter, Linq.list(passes));
    }

    public Passes(String name, DBSPCompiler compiler, List<CircuitTransform> passes) {
        this.compiler = compiler;
        this.passes = passes;
        this.id = CircuitVisitor.crtId++;
        this.name = name;
    }

    @Override
    public DBSPCompiler compiler() {
        return this.compiler;
    }

    public void add(CircuitTransform pass) {
        this.passes.add(pass);
    }

    public void add(InnerRewriteVisitor inner) {
        this.passes.add(new CircuitRewriter(this.compiler(), inner));
    }

    @Override
    public DBSPCircuit apply(DBSPCircuit circuit) {
        int details = this.getDebugLevel();
        if (this.getDebugLevel() >= 3) {
            String name = String.format("%02d-", dumped++) + "before.png";
            ToDot.dump(this.compiler, name, details, "png", circuit);
        }
        for (CircuitTransform pass: this.passes) {
            Logger.INSTANCE.belowLevel("Passes", 1)
                    .append("Executing ")
                    .append(pass.toString())
                    .newline();
            circuit = pass.apply(circuit);
            if (this.getDebugLevel() >= 3) {
                String name = String.format("%02d-", dumped++) + pass.toString().replace(" ", "_") + ".png";
                ToDot.dump(this.compiler, name, details, "png", circuit);
            }
        }
        return circuit;
    }

    @Override
    public String toString() {
        return this.id +
                " " +
                this.name +
                this.passes.size();
    }

    @Override
    public String getName() {
        return this.name;
    }
}
