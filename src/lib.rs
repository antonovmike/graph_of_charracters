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
            "\nGRAPH:\n\tnodes: {:?}\n\tedges: {:?}\n\troot: {:?}\n-----",
            self.nodes, self.edges, self.root
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
        let root = None;
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
    
    fn copy(graph: &Graph) -> Graph {
        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            hash_node.insert(temp_id, temp_node);
        }
        let temp_graph: Graph = Graph { 
            nodes: hash_node, edges: graph.edges.clone(), root: graph.root 
        };
        temp_graph
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
        let temp_graph: Graph = Graph { 
            nodes: hash_node, edges: graph.edges.clone(), root: graph.root 
        };

        temp_graph
    }
    
    pub fn remove_node(graph: &Graph, node: Option<char>, id: Option<u64>) -> Graph {
        if node.is_none() && id.is_none() {
            return Graph::copy(graph)
        }
        
        let mut hash_node: BTreeMap<u64, char> = BTreeMap::new();
        for i in &graph.nodes {
            let temp_id: u64 = *i.0;
            let temp_node: char = *i.1;
            if node.is_some() {
                if temp_node != node.unwrap() {
                    hash_node.insert(temp_id, temp_node);
                }
            } else {  };

            if id.is_some() {
                if temp_id != id.unwrap() {
                    hash_node.insert(temp_id, temp_node);
                }
            } else {  };
        }
        let temp_graph: Graph = Graph { 
            nodes: hash_node, edges: graph.edges.clone(), root: graph.root 
        };

        temp_graph
    }
}
