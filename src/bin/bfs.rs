use graph_of_charracters::*;

fn main() {
    let list_of_nodes = ['A', 'B', 'C', 'D'];

    let empty_graph = Graph::default();
    println!("{}", empty_graph);
    let graph = Graph::add_node(empty_graph, &list_of_nodes);
    let graph = Graph::add_edge(&graph, 'A', 'B');
    let graph = Graph::add_edge(&graph, 'C', 'B');
    let graph = Graph::add_edge(&graph, 'C', 'D');
    println!("v1\n{}", graph);

    let serialized = serde_yaml::to_string(&graph).unwrap();
    println!("serialized\n-----\n{serialized}\n-----\n");

    let graph_2 = Graph::add_node(graph, &['A']);

    let graph_3 = Graph::rm_node(&graph_2, None, Some(1));

    println!("v3\n{}", graph_3);

    let searching_for_name = Graph::get_node_id(&graph_3, 'C');
    println!("Node's ID is {}", searching_for_name.unwrap());
    let searching_for_id = Graph::get_node_name(&graph_3, 2);
    println!("Node's name is {}", searching_for_id.unwrap());

    let edge_name = Graph::get_edge_name(&graph_3, 2, 3);
    println!("Edge name: {}", edge_name);

    println!("search: {}", Graph::search(&graph_3, 'A', 'a'));
}
