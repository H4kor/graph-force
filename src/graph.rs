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

pub fn add_edge(matrix: &mut EdgeMatrix, i: usize, j: usize) {
    let mut edges = matrix.write().unwrap();
    edges[i][j].weight = 1.0;
    edges[j][i].weight = 1.0;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new_edge_matrix() {
        let matrix = new_edge_matrix(5);
        let edges = matrix.read().unwrap();
        assert_eq!(edges.len(), 5);
        for row in edges.iter() {
            assert_eq!(row.len(), 5);
            for edge in row.iter() {
                assert_eq!(edge.weight, 0.0);
            }
        }
    }

    #[test]
    fn test_new_node_vector() {
        let nodes = new_node_vector(5);
        assert_eq!(nodes.len(), 5);
    }

    #[test]
    fn test_add_edge() {
        let mut matrix = new_edge_matrix(5);
        add_edge(&mut matrix, 0, 1);
        let edges = matrix.read().unwrap();
        assert_eq!(edges[0][1].weight, 1.0);
        assert_eq!(edges[1][0].weight, 1.0);

        assert_eq!(edges[0][0].weight, 0.0);
        assert_eq!(edges[1][1].weight, 0.0);
    }
}
