use crate::graph::{EdgeMatrix, NodeVector, Node};

pub struct MyModel {
    edges: EdgeMatrix,
    ranks: Vec<f32>,
    size: usize,
    opt_dist: f32,
    c: f32,
    dc: f32,
}

impl MyModel {

    pub fn new(edges: EdgeMatrix, size: usize, iterations: usize) -> MyModel {
        let opt_dist = 1.0 / (size as f32).sqrt();
        let c = 0.1;
        let mut ranks = vec![0.0; size];
        {
            let edges = edges.read().unwrap();
            for i in 0..size {
                ranks[i] = edges[i].iter().map(|e| e.weight).sum();
            }
        }

        MyModel{
            edges,
            ranks,
            size,
            opt_dist,
            c: c ,
            dc: c / ((iterations + 1) as f32)
        }
    }

    pub fn prepare(& mut self, _nodes: &NodeVector) {
        self.c -= self.dc;
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
            let dist = (d_x * d_x + d_y * d_y).sqrt();
            let unit_x = d_x / dist;
            let unit_y = d_y / dist;

            let edge = edges[i_node][o].weight;

            if edge == 0.0 {
                let f_rep = (self.c / (dist).powi(2)).min(self.c);
                let f_rep_x = f_rep * unit_x;
                let f_rep_y = f_rep * unit_y;

                node_x -= f_rep_x;
                node_y -= f_rep_y;
            } else {
                let f_spring = self.c * 0.5 * (dist - self.opt_dist) / self.ranks[i_node];
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
