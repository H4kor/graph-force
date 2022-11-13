use std::thread;
use std::sync::{Arc, RwLock};
use rand::Rng;

struct Node {
    x: f32,
    y: f32,
}

struct Edge {
    weight: f32,
}

fn nodes_list(size: usize) -> Arc<Vec<RwLock<Node>>> {
    let mut nodes = Vec::new();
    for _ in 0..size {
        let node = RwLock::new(Node {
            x: rand::thread_rng().gen_range(0.0..100.0),
            y: rand::thread_rng().gen_range(0.0..100.0),
        });
        nodes.push(node);
    }
    Arc::new(nodes)
}

fn connection_matrix(size: usize) -> Arc<RwLock<Vec<Vec<Edge>>>> {
    let mut rng = rand::thread_rng();

    let mut matrix = Vec::with_capacity(size);
    for _ in 0..size {
        let mut row = Vec::with_capacity(size);
        for _ in 0..size {
            let p: f32 = rng.gen();
            row.push(Edge { weight: if p < 0.1 {1.0} else {0.0} });
        }
        matrix.push(row);
    }
    Arc::new(RwLock::new(matrix))
}

fn main() {

    let size = 24000;
    let threads = 8;

    let nodes = nodes_list(size);
    let nodes_next = nodes_list(size);

    let edges = connection_matrix(size);

    let mut handles = vec![];

    let chunks = size / threads;
    for i in 0..threads {
        let nodes = nodes.clone();
        let nodes_next = nodes_next.clone();
        let edges = edges.clone();
        let handle = thread::spawn(move || {
            for j in 0..chunks {
                let n = i * chunks + j;
                let node = nodes[n].read().unwrap();
                let edges = edges.read().unwrap();

                let mut node_x = node.x;
                let mut node_y = node.y;

                for o in 0..size {
                    let o_x: f32;
                    let o_y: f32;
                    {
                        let other = nodes[o].read().unwrap();
                        o_x = other.x;
                        o_y = other.y;
                    }
                    let edge = edges[n][o].weight;
                    node_x += (o_x - node.x) * edge;
                    node_y += (o_y - node.y) * edge;
                }
                let mut result = nodes_next[n].write().unwrap();
                result.x = node_x;
                result.y = node_y;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}
