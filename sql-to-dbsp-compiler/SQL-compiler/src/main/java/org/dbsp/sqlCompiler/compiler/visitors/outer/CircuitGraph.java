package org.dbsp.sqlCompiler.compiler.visitors.outer;

import org.dbsp.sqlCompiler.circuit.ICircuit;
import org.dbsp.sqlCompiler.circuit.operator.DBSPOperator;
import org.dbsp.util.IHasId;
import org.dbsp.util.graph.DiGraph;
import org.dbsp.util.graph.Port;
import org.dbsp.util.Utilities;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

/* The Graph represents edges source->destination,
 * while the circuit represents edges destination->source. */
public class CircuitGraph implements DiGraph<DBSPOperator>, IHasId {
    private static long crtId = 0;
    private final long id;
    private final Set<DBSPOperator> nodeSet = new HashSet<>();
    private final List<DBSPOperator> nodes = new ArrayList<>();
    private final Map<DBSPOperator, List<Port<DBSPOperator>>> edges = new HashMap<>();
    /** Circuit whose graph is represented */
    private final ICircuit circuit;

    public CircuitGraph(ICircuit circuit) {
        this.circuit = circuit;
        this.id = crtId++;
    }

    @Override
    public long getId() {
        return this.id;
    }

    void addNode(DBSPOperator node) {
        if (this.nodeSet.contains(node))
            return;
        this.nodes.add(node);
        this.nodeSet.add(node);
        this.edges.put(node, new ArrayList<>());
        assert this.circuit.contains(node);
    }

    void addEdge(DBSPOperator source, DBSPOperator dest, int input) {
        if (!this.nodeSet.contains(source)) {
            throw new RuntimeException(
                    "Adding edge from node " + source + " to " + dest +
                    " when source is not in the graph.");
        }
        if (!this.nodeSet.contains(dest)) {
            throw new RuntimeException(
                    "Adding edge from node " + source + " to " + dest +
                            " when destination is not in the graph.");
        }
        this.edges.get(source).add(new Port<>(dest, input));
    }

    @Override
    public String toString() {
        return "CircuitGraph " + this.id + "(" + this.circuit.getId() + ") {" +
                "nodes=" + this.nodes +
                ", edges=" + this.edges +
                '}';
    }

    public void clear() {
        this.nodeSet.clear();
        this.edges.clear();
        this.nodes.clear();
    }

    @Override
    public Iterable<DBSPOperator> getNodes() {
        return this.nodes;
    }

    public List<Port<DBSPOperator>> getSuccessors(DBSPOperator source) {
        return Utilities.getExists(this.edges, source);
    }
}
