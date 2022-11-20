use crate::graph::{new_edge_matrix, EdgeMatrix, Node, NodeVector};
use crate::model::ForceModel;
pub struct SimpleSpringModel {
    edges: EdgeMatrix,
    size: usize,
    c: f32,
    dc: f32,
}

impl SimpleSpringModel {
    pub fn new(c: f32) -> Self {
        SimpleSpringModel {
            edges: new_edge_matrix(0),
            size: 0,
            c,
            dc: 0.0,
        }
    }
}

impl ForceModel for SimpleSpringModel {
    fn init(&mut self, edges: EdgeMatrix, size: usize, iterations: usize) {
        let dc = self.c / ((iterations + 1) as f32);

        self.edges = edges;
        self.size = size;
        self.dc = dc;
    }

    fn prepare(&mut self, _nodes: &NodeVector) {
        self.c -= self.dc;
    }

    fn step(&self, nodes: &NodeVector, i_node: usize) -> Node {
        let node = nodes[i_node].read().unwrap();
        let edges = self.edges.read().unwrap();

        let node_x = node.x;
        let node_y = node.y;

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for o in 0..self.size {
            if o == i_node {
                continue;
            }
            let o_x: f32;
            let o_y: f32;
            {
                let other = nodes[o].read().unwrap();
                o_x = other.x;
                o_y = other.y;
            }

            let d_x = o_x - node_x;
            let d_y = o_y - node_y;
            let dist = (d_x * d_x + d_y * d_y).sqrt();
            let unit_x = d_x / dist;
            let unit_y = d_y / dist;

            let edge = edges[i_node][o].weight;

            if edge == 0.0 {
                let f_rep = dist.powi(2).recip().min(1.0);
                let f_rep_x = f_rep * unit_x;
                let f_rep_y = f_rep * unit_y;

                sum_x -= f_rep_x;
                sum_y -= f_rep_y;
            } else {
                let f_spring = 0.5 * (dist - 1.0);
                let f_spring_x = f_spring * unit_x;
                let f_spring_y = f_spring * unit_y;
                sum_x += f_spring_x;
                sum_y += f_spring_y;
            }
        }

        // limit the movement
        // TODO: find a good upper bound
        let sum_l = (sum_x * sum_x + sum_y * sum_y).sqrt().max(1e-6).recip() * self.c;
        let sum_x = sum_x * sum_l;
        let sum_y = sum_y * sum_l;

        Node {
            x: node_x + sum_x,
            y: node_y + sum_y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::{new_edge_matrix, new_node_vector};

    #[test]
    fn test_simple_spring_model_attraction() {
        let mut model = SimpleSpringModel::new(1.0);
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
        let mut model = SimpleSpringModel::new(1.0);
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
