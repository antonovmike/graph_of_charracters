use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: BTreeMap<u64, char>,
    edges: Vec<(u64, u64)>,
    root: Option<u64>,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn set_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nGRAPH:\n\tnodes: {:?}\n\tedges: {:?}\n\troot:  {:?}\n-----",
            self.nodes,
            self.edges,
            self.root.unwrap()
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
        let root = Some(0);
        Graph { nodes, edges, root }
    }

    pub fn get_node_id(graph: &Graph, node: char) -> Option<u64> {
        let mut node_id: Option<u64> = None;
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if node == temp_node {
                node_id = Some(temp_id)
            };
        }

        node_id
    }

    pub fn get_node_name(graph: &Graph, id: u64) -> Option<char> {
        let mut node_name: Option<char> = None;
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if id == temp_id {
                node_name = Some(temp_node)
            };
        }

        node_name
    }

    pub fn get_edge_name(graph: &Graph, start_id: u64, end_id: u64) -> String {
        let tupple = (start_id, end_id);
        if !graph.edges.contains(&tupple) {
            return "Graph doesn't contan this Edge".to_string();
        } else {
            let first = Graph::get_node_name(&graph, start_id);
            let last = Graph::get_node_name(&graph, end_id);
            return format!("{} –> {}", first.unwrap(), last.unwrap());
        };
    }

    fn copy(graph: &Graph) -> Graph {
        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            hash_node.insert(temp_id, temp_node);
        }

        Graph {
            nodes: hash_node,
            edges: graph.edges.clone(),
            root: graph.root,
        }
    }

    pub fn add_node(graph: &Graph, node: &[char]) -> Graph {
        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            hash_node.insert(temp_id, temp_node);
        }

        for i in node {
            let check = Graph::get_node_id(&graph, *i).is_none();
            if check == true {
                let id = set_id() as u64;
                hash_node.insert(id, *i);
            }
        }

        Graph {
            nodes: hash_node,
            edges: graph.edges.clone(),
            root: graph.root,
        }
    }

    pub fn rm_node(graph: &Graph, node: Option<char>, id: Option<u64>) -> Graph {
        if node.is_none() && id.is_none() {
            return Graph::copy(graph);
        }

        let mut temp_vec: Vec<(u64, u64)> = graph.edges.clone();

        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if node.is_some() {
                if temp_node != node.unwrap() {
                    hash_node.insert(temp_id, temp_node);
                } else {
                    let node_to_remove = Graph::rm_edge_by_node(&graph, node.unwrap()).edges;
                    temp_vec = node_to_remove;
                }
            };

            if id.is_some() {
                if temp_id != id.unwrap() {
                    hash_node.insert(temp_id, temp_node);
                } else {
                    let node_by_id = Graph::get_node_name(&graph, id.unwrap()).unwrap();
                    let node_to_remove = Graph::rm_edge_by_node(&graph, node_by_id).edges;
                    temp_vec = node_to_remove;
                }
            } else {
            };
        }

        Graph {
            nodes: hash_node,
            edges: temp_vec,
            root: graph.root,
        }
    }

    fn check_neighbors(graph: &Graph, start_node: char, end_node: char) -> bool {
        let first_n_id = Graph::get_node_id(&graph, start_node).unwrap();
        let last_n_id = Graph::get_node_id(&graph, end_node).unwrap();
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
            root: graph.root,
        }
    }

    pub fn rm_edge_by_node(graph: &Graph, node: char) -> Graph {
        let mut temp_vec: Vec<(u64, u64)> = graph.edges.clone();
        let node_id = Graph::get_node_id(&graph, node).unwrap();

        for i in temp_vec.clone() {
            if i.0 == node_id || i.1 == node_id {
                let index = temp_vec.iter().position(|&r| r == i).unwrap();
                temp_vec.remove(index);
            }
        }

        Graph {
            nodes: graph.nodes.clone(),
            edges: temp_vec,
            root: graph.root,
        }
    }

    pub fn rm_edge(graph: &Graph, start_node: char, end_node: char) -> Graph {
        let mut temp_vec = graph.edges.clone();
        let first_id = Graph::get_node_id(&graph, start_node).unwrap();
        let last_id = Graph::get_node_id(&graph, end_node).unwrap();
        let tupple = (first_id, last_id);

        temp_vec.retain(|&x| x != tupple);

        Graph {
            nodes: graph.nodes.clone(),
            edges: temp_vec,
            root: graph.root,
        }
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
            root: Some(0),
        };
        let new_graph = Graph::default();
        assert_eq!(empty_graph.nodes, new_graph.nodes);
        assert_eq!(empty_graph.edges, new_graph.edges);
        assert_eq!(empty_graph.root, new_graph.root);
    }

    #[test]
    fn same_node_id() {
        let list_of_nodes = ['a', 'b', 'c', 'd'];
        let graph = Graph::add_node(&Graph::default(), &list_of_nodes);

        for n in list_of_nodes {
            Graph::add_node(&graph, &[n]);
        }

        assert_eq!(false, Graph::check_neighbors(&graph, 'd', 'd'));
    }
}
