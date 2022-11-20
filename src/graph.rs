use rand::Rng;
use std::sync::{Arc, RwLock};
pub struct Node {
    pub x: f32,
    pub y: f32,
}

pub struct Edge {
    pub weight: f32,
}

pub type EdgeMatrix = Arc<RwLock<Vec<Vec<Edge>>>>;
pub type NodeVector = Arc<Vec<RwLock<Node>>>;

pub fn new_edge_matrix(size: usize) -> EdgeMatrix {
    let mut matrix = Vec::with_capacity(size);
    for _ in 0..size {
        let mut row = Vec::with_capacity(size);
        for _ in 0..size {
            row.push(Edge { weight: 0.0 });
        }
        matrix.push(row);
    }
    Arc::new(RwLock::new(matrix))
}

pub fn new_node_vector(size: usize) -> NodeVector {
    let mut nodes = Vec::with_capacity(size);
    for _ in 0..size {
        nodes.push(RwLock::new(Node {
            x: rand::thread_rng().gen_range(-0.5..0.5),
            y: rand::thread_rng().gen_range(-0.5..0.5),
        }));
    }
    Arc::new(nodes)
}
