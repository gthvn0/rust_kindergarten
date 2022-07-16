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
        // As edges are not directed we also add dst -> src
        self.nodes[dst].neighbours.push(src);
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
            n.visited = false;
        }

        fifo.push(start);
        self.nodes[start].visited = true;
        println!("Start node {:?}", start);

        while fifo_head + 1 <= fifo.len() {
            let neighbours: Vec<usize>;

            {
                // When looping through neighbours we will need to have a mutable
                // reference to self.nodes. So we create a scope to get the list
                // of neighbours and don't have issue with the current_node that
                // also have a reference to self.nodes.
                let current_id = fifo[fifo_head];
                let current_node = &self.nodes[current_id];
                println!("Processing node {} <{:?}>", current_id, current_node);
                neighbours = current_node.neighbours.clone();
            }

            for id in neighbours.iter() {
                let neighbour: &mut Node = &mut self.nodes[*id];
                if !neighbour.visited {
                    fifo.push(*id);
                    neighbour.visited = true;
                }
                println!("    -> Added neighbour {} <{:?}>", *id, neighbour);
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
    // 3         6
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


    g.display();

    g.bfs(2);
}
