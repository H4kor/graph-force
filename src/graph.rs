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
