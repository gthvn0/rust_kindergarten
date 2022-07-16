use std::cmp::max;

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
    fn add_edge(&mut self, src: usize, dst: usize) {
        let m: usize = max(src, dst) + 1; // used to check if new nodes need to be created

        // If len is greater or equla to m nothing is pushed
        for _ in self.nodes.len()..m {
            self.nodes.push(Node::default());
        }
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
    //      0
    //     / \
    //    1   2
    //   /|   |\
    //  / 4---5 \
    // 3         6
    // Create a graph
    let mut g: Graph = Graph::default();

    // Add some edges
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(1, 4);
    g.add_edge(2, 5);
    g.add_edge(2, 6);
    g.add_edge(4, 5);


    g.display();
}
