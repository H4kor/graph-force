use crate::graph::{EdgeMatrix, Node, NodeVector};

pub trait ForceModel {
    fn init(&mut self, edges: EdgeMatrix, size: usize, iterations: usize);
    fn prepare(&mut self, _nodes: &NodeVector);
    fn step(&self, nodes: &NodeVector, i_node: usize) -> Node;
}
