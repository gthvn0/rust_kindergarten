#[derive(Debug, Default)]
struct Node {
    neighbours: Vec<i32>,
    visited: bool,
}

type Graph = Vec<Node>;

fn main() {
    let mut g: Graph = Graph::new();


    // Start with simple graph
    //     0
    //    / \
    //   1   2
    //  /
    // 3

    // Start by creating the four nodes
    g.push(Node::default());
    g.push(Node::default());
    g.push(Node::default());
    g.push(Node::default());

    // Add the edges
    g[0].neighbours.push(1);
    g[0].neighbours.push(2);
    g[1].neighbours.push(3);

    // Print the graph
    for (i, n) in g.iter().enumerate() {
        println!("Node {}: {:?}", i, n);
    }
}
