use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct GraphHasCycles;

#[derive(Debug, Clone)]
pub struct DirectedGraph<R> where R: Hash + Eq + Clone + Copy {
    nodes: HashMap<R, Node<R>>,
}

impl<R> Default for DirectedGraph<R> where R: Hash + Eq + Clone+ Copy {
    fn default() -> Self {
        DirectedGraph {
            nodes: HashMap::new()
        }
    }
}

impl<R> DirectedGraph<R> where R: Hash + Eq + Clone + Copy {
    pub fn add_edge(&mut self, from_reference: R, to_reference: R) {
        self._add_outgoing_edge(from_reference, to_reference);
        self._add_incoming_edge(from_reference, to_reference);
    }

    pub fn remove_edge(&mut self, from_reference: R, to_reference: R) {
        self._remove_outgoing_edge(from_reference, to_reference);
        self._remove_incoming_edge(from_reference, to_reference);
    }

    pub fn set_incoming_edges(
        &mut self,
        reference: R,
        new_incoming_edges: HashSet<R>,
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

    pub fn to_topological_sort(&self) -> Result<Vec<R>, GraphHasCycles> {
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

    fn get(&self, reference: R) -> Option<&Node<R>> {
        self.nodes.get(&reference)
    }

    fn get_mut(&mut self, reference: R) -> Option<&mut Node<R>> {
        self.nodes.get_mut(&reference)
    }

    fn get_or_default_mut(&mut self, reference: R) -> &mut Node<R> {
        self.nodes.entry(reference).or_insert_with(Node::default)
    }

    fn get_references_with_no_incoming_edges(&self) -> Vec<R> {
        self.nodes
            .iter()
            .filter(|(_, n)| !n.has_incoming_edges())
            .map(|(r, _)| r.clone())
            .collect()
    }

    // WARNING: these do not maintain the invariant of both relationships being correctly updated

    fn _add_outgoing_edge(&mut self, from_reference: R, to_reference: R) {
        self.get_or_default_mut(from_reference)
            .outgoing_edges
            .insert(to_reference);
    }

    fn _remove_outgoing_edge(&mut self, from_reference: R, to_reference: R) {
        if let Some(node) = self.get_mut(from_reference) {
            node.outgoing_edges.remove(&to_reference);
        }
    }

    fn _add_incoming_edge(&mut self, from_reference: R, to_reference: R) {
        self.get_or_default_mut(to_reference)
            .incoming_edges
            .insert(from_reference);
    }

    fn _remove_incoming_edge(&mut self, from_reference: R, to_reference: R) {
        if let Some(node) = self.get_mut(to_reference) {
            node.incoming_edges.remove(&from_reference);
        }
    }
}

#[derive(Debug, Clone)]
struct Node<R> where R: Hash + Eq + Clone {
    outgoing_edges: HashSet<R>,
    incoming_edges: HashSet<R>,
}


impl<R> Default for Node<R> where R: Hash + Eq + Clone{
    fn default() -> Self {
        Node {
            outgoing_edges: HashSet::new(),
            incoming_edges: HashSet::new()
        }
    }
}

impl<R> Node<R> where R: Hash + Eq + Clone {
    fn has_incoming_edges(&self) -> bool {
        !self.incoming_edges.is_empty()
    }

    fn has_outgoing_edges(&self) -> bool {
        !self.outgoing_edges.is_empty()
    }
}
