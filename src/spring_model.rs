use crate::graph::{EdgeMatrix, NodeVector, Node};


const C_REP: f32 = 0.1;
const C_SPRING: f32 = 0.1;

pub struct InitialModel {
    edges: EdgeMatrix,
    size: usize,
    opt_dist: f32,
    t: f32,

}

impl InitialModel {

    pub fn new(edges: EdgeMatrix, size: usize) -> InitialModel {
        let opt_dist = 1.0 / (size as f32).sqrt();
        InitialModel{ edges, size, opt_dist, t: 0.1 }
    }

    pub fn prepare(& mut self, nodes: &NodeVector) {
        self.t = 0.1 * f32::max(
            nodes.iter().map(|n| n.read().unwrap().x.abs()).reduce(|a, b| a.max(b)).unwrap(),
            nodes.iter().map(|n| n.read().unwrap().y.abs()).reduce(|a, b| a.max(b)).unwrap()
        );
    }

    pub fn step(&self, nodes: &NodeVector, i_node: usize) -> Node {

        let node = nodes[i_node].read().unwrap();
        let edges = self.edges.read().unwrap();

        let mut node_x = node.x;
        let mut node_y = node.y;
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
            let dist = (d_x * d_x + d_y * d_y).sqrt().max(0.01);
            let unit_x = d_x / dist;
            let unit_y = d_y / dist;

            let edge = edges[i_node][o].weight;

            if edge == 0.0 {
                let f_rep = (C_REP / (dist).powi(2)).min(self.t);
                let f_rep_x = f_rep * unit_x;
                let f_rep_y = f_rep * unit_y;

                node_x -= f_rep_x;
                node_y -= f_rep_y;
            } else {
                let f_spring = (C_SPRING * (dist / self.opt_dist).log(2.0)).min(self.t);
                let f_spring_x = f_spring * unit_x;
                let f_spring_y = f_spring * unit_y;
                node_x += f_spring_x;
                node_y += f_spring_y;
            }
        }
        Node {
            x: node_x,
            y: node_y,
        }        
    }
}
