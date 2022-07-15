#[derive(Debug, Default)]
struct Node {
    neighbours: Vec<usize>,
    visited: bool,
}

#[derive(Default)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    // Return a new graph with n nodes
    fn new(n: usize) -> Self {
        let mut g: Graph = Graph::default();

        for _ in 0..n {
            g.nodes.push(Node::default());
        }
        g
    }

    fn add_edge(&mut self, src: usize, dst: usize) {
        self.nodes[src].neighbours.push(dst);
    }

    fn display(&self) {
        for (i, n) in self.nodes.iter().enumerate() {
            println!("Node {}: {:?}", i, n);
        } 
    }
}
fn main() {
    // Start with simple graph
    //     0
    //    / \
    //   1   2
    //  /
    // 3

    // Create a graph with four nodes
    let mut g: Graph = Graph::new(4);

    // Add the edges
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);

    g.display();
}
