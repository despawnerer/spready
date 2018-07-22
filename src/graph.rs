use std::collections::HashMap;
use std::collections::HashSet;

use reference::Reference;

#[derive(Debug)]
pub struct GraphHasCycles;

#[derive(Debug, Clone, Default)]
pub struct DirectedGraph {
    nodes: HashMap<Reference, Node>,
}

impl DirectedGraph {
    pub fn add_edge(&mut self, from_reference: Reference, to_reference: Reference) {
        self._add_outgoing_edge(from_reference, to_reference);
        self._add_incoming_edge(from_reference, to_reference);
    }

    pub fn remove_edge(&mut self, from_reference: Reference, to_reference: Reference) {
        self._remove_outgoing_edge(from_reference, to_reference);
        self._remove_incoming_edge(from_reference, to_reference);
    }

    pub fn set_incoming_edges(
        &mut self,
        reference: Reference,
        new_incoming_edges: HashSet<Reference>,
    ) {
        let previous_incoming_edges = self.get_or_default_mut(reference).incoming_edges.clone();

        for added_edge in new_incoming_edges.difference(&previous_incoming_edges) {
            self._add_outgoing_edge(*added_edge, reference);
        }

        for removed_edge in previous_incoming_edges.difference(&new_incoming_edges) {
            self._remove_outgoing_edge(*removed_edge, reference);
        }

        self.get_or_default_mut(reference).incoming_edges = new_incoming_edges;
    }

    pub fn has_edges(&self) -> bool {
        self.nodes
            .values()
            .any(|n| n.has_incoming_edges() || n.has_outgoing_edges())
    }

    pub fn to_topological_sort(&self) -> Result<Vec<Reference>, GraphHasCycles> {
        let mut graph = self.clone();

        let mut sorted_references = Vec::new();
        let mut independent_references = graph.get_references_with_no_incoming_edges();

        while !independent_references.is_empty() {
            let reference = independent_references.pop().unwrap();
            let node = graph.get(reference).unwrap().clone();
            sorted_references.push(reference);
            for outgoing_reference in &node.outgoing_edges {
                graph.remove_edge(reference, *outgoing_reference);
                let outgoing_node = graph.get(*outgoing_reference).unwrap();
                if !outgoing_node.has_incoming_edges() {
                    independent_references.push(*outgoing_reference);
                }
            }
        }

        if graph.has_edges() {
            Err(GraphHasCycles)
        } else {
            Ok(sorted_references)
        }
    }

    fn get(&self, reference: Reference) -> Option<&Node> {
        self.nodes.get(&reference)
    }

    fn get_mut(&mut self, reference: Reference) -> Option<&mut Node> {
        self.nodes.get_mut(&reference)
    }

    fn get_or_default_mut(&mut self, reference: Reference) -> &mut Node {
        self.nodes.entry(reference).or_insert_with(Node::default)
    }

    fn get_references_with_no_incoming_edges(&self) -> Vec<Reference> {
        self.nodes
            .iter()
            .filter(|(_, n)| !n.has_incoming_edges())
            .map(|(r, _)| r.clone())
            .collect()
    }

    // WARNING: these do not maintain the invariant of both relationships being correctly updated

    fn _add_outgoing_edge(&mut self, from_reference: Reference, to_reference: Reference) {
        self.get_or_default_mut(from_reference)
            .outgoing_edges
            .insert(to_reference);
    }

    fn _remove_outgoing_edge(&mut self, from_reference: Reference, to_reference: Reference) {
        if let Some(node) = self.get_mut(from_reference) {
            node.outgoing_edges.remove(&to_reference);
        }
    }

    fn _add_incoming_edge(&mut self, from_reference: Reference, to_reference: Reference) {
        self.get_or_default_mut(to_reference)
            .incoming_edges
            .insert(from_reference);
    }

    fn _remove_incoming_edge(&mut self, from_reference: Reference, to_reference: Reference) {
        if let Some(node) = self.get_mut(to_reference) {
            node.incoming_edges.remove(&from_reference);
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    outgoing_edges: HashSet<Reference>,
    incoming_edges: HashSet<Reference>,
}

impl Node {
    fn has_incoming_edges(&self) -> bool {
        !self.incoming_edges.is_empty()
    }

    fn has_outgoing_edges(&self) -> bool {
        !self.outgoing_edges.is_empty()
    }
}
