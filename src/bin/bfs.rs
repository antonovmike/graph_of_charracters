use graph_of_charracters::*;

fn main() {
    println!("The Graph of Characters");

    let list_of_nodes = ['A', 'B', 'C', 'D'];

    let empty_graph = Graph::default();
    let graph = Graph::add_node(&empty_graph, &list_of_nodes);
    let graph = Graph::add_edge(&graph, 'A', 'B');
    let graph = Graph::add_edge(&graph, 'C', 'B');    
    // println!("v1\n{}", graph);
    
    let graph_2 = Graph::add_node(&graph, &['A']);
    
    // println!("v2\n{}", graph_2);
    
    let graph_3 = Graph::remove_node(&graph_2, None, Some(1));
        
    println!("v3\n{}", graph_3);
    
    let searching_for_name = Graph::get_node_id(&graph_3, 'C');
    println!("Node's  ID  is {:?}", searching_for_name);
    let searching_for_id = Graph::get_node_name(&graph_3, 2);
    println!("Node's name is {:?}", searching_for_id);
    
    let edge_name = Graph::get_edge_name(&graph_3, 2, 9);
    println!("{:?}", edge_name);
}
