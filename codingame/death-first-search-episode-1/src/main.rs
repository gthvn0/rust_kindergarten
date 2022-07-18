use std::cmp::max;

#[derive(Debug, Default)]
struct Node {
    neighbours: Vec<usize>,
    path: Vec<usize>,
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

        if !self.nodes[src].neighbours.contains(&dst) {
            self.nodes[src].neighbours.push(dst);
            // As edges are not directed we also add dst -> src
            self.nodes[dst].neighbours.push(src);
        }
    }

    fn display(&self) {
        for (i, n) in self.nodes.iter().enumerate() {
            println!("Node {}: {:?}", i, n);
        }
    }

    // Breadth-First Search Algorithm. Parcours en largeur
    // We will use a FIFO. We don't really remove the first item but we
    // move the head:
    //
    //   +-+-+-+-+-+---
    //   |1|2|5|3|4|    <= Added new item (FIFO Len 5)
    //   +-+-+-+-+-+---
    //    ^
    //    |
    //    fifo_head = 0
    fn bfs(&mut self, start: usize) {
        let mut fifo: Vec<usize> = Vec::new();
        let mut fifo_head: usize = 0;
        // 1. Put the start node in the fifo
        // 2. WHILE: not all nodes ID in the fifo have been processed
        //      DO
        //          - Process the head of the fifo
        //              - Put not visited neighbours in the FIFO
        //              - Note them as visited... et voilà
        //      DONE

        // Start by setting all nodes as not visisted
        for n in self.nodes.iter_mut() {
            n.path.clear();
            n.visited = false;
        }

        // Push the node used to start the visit
        fifo.push(start);
        self.nodes[start].visited = true;
        //eprintln!("Start node {:?}", start);

        // the main loop
        while fifo_head + 1 <= fifo.len() {
            let neighbours: Vec<usize>; // Copy of the list of neighbours
            let current_path: Vec<usize>; // Copy of the current path

            // we need to use copy to avoid to have a mutable reference on
            // self.nodes while iterating the main loop.
            {
                let current_id = fifo[fifo_head];
                let current_node = &self.nodes[current_id];
                //eprintln!("    => processing node {}", current_id);
                neighbours = current_node.neighbours.clone();
                current_path = current_node.path.clone();
            }

            for id in neighbours.iter() {
                let neighbour: &mut Node = &mut self.nodes[*id];
                if !neighbour.visited {
                    //eprintln!("        => Visiting node {}", id);
                    // Update path and visited state. I don't really like this
                    // cloning. Probably a better way exists...
                    neighbour.path = current_path.clone();
                    neighbour.path.push(fifo[fifo_head]);
                    neighbour.visited = true;
                    fifo.push(*id);
                    //eprintln!("        => Pushing node {}", id);
                }
            }

            // We can now take the next value. At some point all nodes are visited
            // and nothing is pushed so current will catch the size of the fifo.
            fifo_head += 1;
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
    // 3         6 - 7
    //
    // Create a graph
    let mut g: Graph = Graph::default();

    // Add some edges. Edges are not directed
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(1, 4);
    g.add_edge(2, 5);
    g.add_edge(2, 6);
    g.add_edge(4, 5);
    g.add_edge(6, 7);

    g.bfs(2);
    g.display();
}
