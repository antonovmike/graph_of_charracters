use graph_of_charracters::*;

fn main() {
    println!("The Graph of Characters");

    let list_of_nodes = ['A', 'B', 'C', 'D'];

    let empty_graph = Graph::default();
    let graph = Graph::add_node(&empty_graph, &list_of_nodes);
    
    println!("v1\n{}", graph);
    
    let graph_2 = Graph::add_node(&graph, &['E']);
    
    println!("v2\n{}", graph_2);
    
    let graph_3 = Graph::remove_node(&graph_2, None, Some(1));
        
    println!("v3\n{}", graph_3);
}
