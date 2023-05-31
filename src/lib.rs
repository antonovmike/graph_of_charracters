use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{
    collections::BTreeMap,
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug)]
struct EdgeName;

impl fmt::Display for EdgeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't get Edge name")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    nodes: BTreeMap<u64, char>,
    edges: Vec<(u64, u64)>,
}

impl IntoIterator for Graph {
    type Item = (u64, u64);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // self.nodes.into_iter()
        self.edges.into_iter()
    }
}

impl Serialize for Graph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Graph", 3)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.serialize_field("edges", &self.edges)?;
        state.end()
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn set_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nGRAPH:\n\tnodes: {:?}\n\tedges: {:?}\n-----",
            self.nodes, self.edges,
        )
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        let nodes: BTreeMap<u64, char> = BTreeMap::new();
        let edges = vec![];
        Graph { nodes, edges }
    }

    pub fn get_node_id(&self, node: char) -> Option<u64> {
        let mut node_id: Option<u64> = None;
        for i in &self.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if node == temp_node {
                node_id = Some(temp_id)
            };
        }

        node_id
    }

    pub fn get_node_name(&self, id: u64) -> Option<char> {
        let mut node_name: Option<char> = None;
        for i in &self.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if id == temp_id {
                node_name = Some(temp_node)
            };
        }

        node_name
    }

    pub fn get_edge_name(&self, start_id: u64, end_id: u64) -> String {
        let tupple = (start_id, end_id);
        if !self.edges.contains(&tupple) {
            return "Graph doesn't contan this Edge".to_string();
        } else {
            let first = Graph::get_node_name(&self, start_id);
            let last = Graph::get_node_name(&self, end_id);
            return format!("{} –> {}", first.unwrap(), last.unwrap());
        };
    }

    fn copy(&self) -> Graph {
        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &self.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            hash_node.insert(temp_id, temp_node);
        }

        Graph {
            nodes: hash_node,
            edges: self.edges.clone(),
        }
    }

    pub fn add_node(self, node: &[char]) -> Graph {
        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &self.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            hash_node.insert(temp_id, temp_node);
        }

        for i in node {
            let check = Graph::get_node_id(&self, *i).is_none();
            if check == true {
                let id = set_id() as u64;
                hash_node.insert(id, *i);
            }
        }

        Graph {
            nodes: hash_node,
            edges: self.edges.clone(),
        }
    }

    pub fn rm_node(&self, node: Option<char>, id: Option<u64>) -> Graph {
        if node.is_none() && id.is_none() {
            return Graph::copy(self);
        }

        let mut temp_vec: Vec<(u64, u64)> = self.edges.clone();

        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &self.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if node.is_some() {
                if temp_node != node.unwrap() {
                    hash_node.insert(temp_id, temp_node);
                } else {
                    let node_to_remove = Graph::rm_edge_by_node(&self, node.unwrap()).edges;
                    temp_vec = node_to_remove;
                }
            };

            if id.is_some() {
                if temp_id != id.unwrap() {
                    hash_node.insert(temp_id, temp_node);
                } else {
                    let node_by_id = Graph::get_node_name(&self, id.unwrap()).unwrap();
                    let node_to_remove = Graph::rm_edge_by_node(&self, node_by_id).edges;
                    temp_vec = node_to_remove;
                }
            } else {
            };
        }

        Graph {
            nodes: hash_node,
            edges: temp_vec,
        }
    }

    fn check_neighbors(&self, start_node: char, end_node: char) -> bool {
        let first_n_id = Graph::get_node_id(&self, start_node).unwrap();
        let last_n_id = Graph::get_node_id(&self, end_node).unwrap();
        if first_n_id == last_n_id {
            return false;
        } else {
            return true;
        }
        // for the previous Edge
        // only one smallest neighbor is allowed
        // for the next two Edges
        // only two bigger neighbors are allowed
    }

    // Each Node can be a part of 1 previous Edge and 2 next Edges
    pub fn add_edge(graph: &Graph, start_node: char, end_node: char) -> Graph {
        if Graph::check_neighbors(graph, start_node, end_node) {
            println!("Check ID\tTrue")
        } else {
            println!("Check ID\tFalse")
        }
        let first = Graph::get_node_id(&graph, start_node);
        let last = Graph::get_node_id(&graph, end_node);

        let mut vector: Vec<(u64, u64)> = graph.edges.clone();

        if first.is_some() && last.is_some() && first != last {
            vector.push((first.unwrap(), last.unwrap()))
        }

        Graph {
            nodes: graph.nodes.clone(),
            edges: vector,
        }
    }

    pub fn rm_edge_by_node(&self, node: char) -> Graph {
        let mut temp_vec: Vec<(u64, u64)> = self.edges.clone();
        let node_id = Graph::get_node_id(&self, node).unwrap();

        for i in temp_vec.clone() {
            if i.0 == node_id || i.1 == node_id {
                let index = temp_vec.iter().position(|&r| r == i).unwrap();
                temp_vec.remove(index);
            }
        }

        Graph {
            nodes: self.nodes.clone(),
            edges: temp_vec,
        }
    }

    pub fn rm_edge(&self, start_node: char, end_node: char) -> Graph {
        let mut temp_vec = self.edges.clone();
        let first_id = Graph::get_node_id(&self, start_node).unwrap();
        let last_id = Graph::get_node_id(&self, end_node).unwrap();
        let tupple = (first_id, last_id);

        temp_vec.retain(|&x| x != tupple);

        Graph {
            nodes: self.nodes.clone(),
            edges: temp_vec,
        }
    }

    pub fn search(&self, start: char, end: char) -> bool {
        // let queue 
        for node in self.nodes.iter() {
            if node.1 == &start {
                return true;
            }
        }
        false
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_graph() {
        let empty_graph = Graph {
            nodes: BTreeMap::new(),
            edges: vec![],
        };
        let new_graph = Graph::default();
        assert_eq!(empty_graph.nodes, new_graph.nodes);
        assert_eq!(empty_graph.edges, new_graph.edges);
    }

    #[test]
    fn same_node_id() {
        let list_of_nodes = ['a', 'b', 'c', 'd'];
        let graph = Graph::add_node(Graph::default(), &list_of_nodes);

        for n in list_of_nodes {
            Graph::add_node(graph.clone(), &[n]);
        }

        assert_eq!(false, Graph::check_neighbors(&graph, 'd', 'd'));
    }
}
