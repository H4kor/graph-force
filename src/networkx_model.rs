use crate::graph::{new_edge_matrix, EdgeMatrix, Node, NodeVector};
use crate::model::ForceModel;

/**
 * Translation of the NetworkX spring_layout function.
 */
pub struct NetworkXModel {
    edges: EdgeMatrix,
    size: usize,
    k: f32,  // optimal distance
    t: f32,  // temperature
    dt: f32, // temperature decrease
}

impl NetworkXModel {
    pub fn new() -> Self {
        NetworkXModel {
            edges: new_edge_matrix(0),
            size: 0,
            k: 1.0,
            t: 1.0,
            dt: 0.0,
        }
    }
}

impl ForceModel for NetworkXModel {
    fn init(&mut self, edges: EdgeMatrix, size: usize, iterations: usize) {
        self.k = (1.0 / size as f32).sqrt();
        self.t = 0.1;
        self.dt = self.t / ((iterations + 1) as f32);
        self.t += self.dt; // prepare is called before the first step
        self.edges = edges;
        self.size = size;
    }

    fn prepare(&mut self, _nodes: &NodeVector) {
        self.t -= self.dt; // decrease temperature
    }

    fn step(&self, nodes: &NodeVector, i_node: usize) -> Node {
        let node = nodes[i_node].read().unwrap();
        let edges = self.edges.read().unwrap();

        let node_x = node.x;
        let node_y = node.y;

        let mut displacement_x = 0.0;
        let mut displacement_y = 0.0;

        for o in 0..self.size {
            if o == i_node {
                continue;
            }

            let edge = edges[i_node][o].weight;

            let o_x: f32;
            let o_y: f32;
            {
                let other = nodes[o].read().unwrap();
                o_x = other.x;
                o_y = other.y;
            }
            // difference between node and other
            let delta_x = node_x - o_x;
            let delta_y = node_y - o_y;
            // distance between node and other
            let dist = (delta_x * delta_x + delta_y * delta_y).sqrt();
            // enforce minimum distance of 0.01
            let dist = if dist < 0.01 { 0.01 } else { dist };

            // displacement "force"
            displacement_x += delta_x * (self.k.powi(2) / dist.powi(2) - edge * dist / self.k);
            displacement_y += delta_y * (self.k.powi(2) / dist.powi(2) - edge * dist / self.k);
        }
        // update positions
        let length = (displacement_x * displacement_x + displacement_y * displacement_y).sqrt();
        let length = if length < 0.01 { 0.01 } else { length };
        let delta_pos_x = displacement_x * self.t / length;
        let delta_pos_y = displacement_y * self.t / length;

        Node {
            x: node_x + delta_pos_x,
            y: node_y + delta_pos_y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::{new_edge_matrix, new_node_vector};

    #[test]
    fn test_simple_spring_model_attraction() {
        let mut model = NetworkXModel::new();
        let edges = new_edge_matrix(2);
        edges.write().unwrap()[0][1].weight = 1.0;
        edges.write().unwrap()[1][0].weight = 1.0;
        model.init(edges, 2, 1);

        let nodes = new_node_vector(2);
        nodes[0].write().unwrap().x = 0.0;
        nodes[0].write().unwrap().y = 0.0;
        nodes[1].write().unwrap().x = 1.0;
        nodes[1].write().unwrap().y = 1.0;

        model.prepare(&nodes);
        let node = model.step(&nodes, 0);
        assert!(node.x > 0.0);
        assert!(node.y > 0.0);
    }

    #[test]
    fn test_simple_spring_model_repulsion() {
        let mut model = NetworkXModel::new();
        let edges = new_edge_matrix(2);
        model.init(edges, 2, 1);

        let nodes = new_node_vector(2);
        nodes[0].write().unwrap().x = 0.0;
        nodes[0].write().unwrap().y = 0.0;
        nodes[1].write().unwrap().x = 1.0;
        nodes[1].write().unwrap().y = 1.0;

        model.prepare(&nodes);
        let node = model.step(&nodes, 0);
        assert!(node.x < 0.0);
        assert!(node.y < 0.0);
    }
}
