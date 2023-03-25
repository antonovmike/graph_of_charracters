# graph_of_charracters

Type of Nodes is char (charracter)

I tried to implement [generic Graph](https://github.com/antonovmike/graph_library) but it is too difficult. I'll do this task first, and then I'll go back to the previous one

Methods:
```Rust
// Create new empty Graph
Graph::default()
// Add Node or Nodes
Graph::add_node(&graph, &['F', 'W']);
// Remove Node by it's' name
Graph::remove_node(&graph_2, Some('W'), None);
// Remove Node by it's 'id
Graph::remove_node(&graph_2, None, Some(1));
```

**TODO:**

- Each Node can be a part of 1 previous Edge and 2 next Edges

- Breadth-first search (or any other algorithm);
- Serialization into trivial Graph Format;
- Deserialization;
- Add tests.

**DONE**
- Add/remove Nodes;
- Add/remove directed edges;
- Remove Edges if Node is removed

For example
```rust
struct Graph<N, E> {
    nodes: HashMap<u64, char>,
    edges: HashMap<u64, HashMap<u64, char>>,
}
```

[Graph Data Structure And Algorithms](https://www.geeksforgeeks.org/graph-data-structure-and-algorithms/)
