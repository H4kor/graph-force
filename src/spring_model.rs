use crate::graph::{EdgeMatrix, Node, NodeVector};

pub struct MyModel {
    edges: EdgeMatrix,
    size: usize,
    opt_dist: f32,
    c: f32,
    dc: f32,
}

impl MyModel {
    pub fn new(edges: EdgeMatrix, size: usize, iterations: usize) -> MyModel {
        let opt_dist = 1.0;
        let c = 0.1;

        MyModel {
            edges,
            size,
            opt_dist,
            c: c,
            dc: c / ((iterations + 1) as f32),
        }
    }

    pub fn prepare(&mut self, _nodes: &NodeVector) {
        self.c -= self.dc;
    }

    pub fn step(&self, nodes: &NodeVector, i_node: usize) -> Node {
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
                let f_rep = dist.powi(2).recip().min(self.opt_dist);
                let f_rep_x = f_rep * unit_x;
                let f_rep_y = f_rep * unit_y;

                sum_x -= f_rep_x;
                sum_y -= f_rep_y;
            } else {
                let f_spring = 0.5 * (dist - self.opt_dist);
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
