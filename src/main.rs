use std::thread;
use std::sync::{Arc, RwLock};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

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

fn main()  -> std::io::Result<()> {

    const C_REP: f32 = 0.1;
    const C_SPRING: f32 = 0.1;
    const ITER: usize = 200;

    let size = 5000;
    let threads = 8;

    let nodes = nodes_list(size);
    let nodes_next = nodes_list(size);

    let edges = connection_matrix(size);

    for epoch in 0..ITER {
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
                        if o == n {
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
                        
                        let f_rep = C_REP/(dist).powi(2);
                        let f_rep_x = f_rep * unit_x;
                        let f_rep_y = f_rep * unit_y;
    
                        node_x += f_rep_x;
                        node_y += f_rep_y;
    
                        let edge = edges[n][o].weight;
                        if edge > 0.0 {
                            let f_spring = C_SPRING * (dist / edge).log(2.0);
                            let f_spring_x = f_spring * unit_x;
                            let f_spring_y = f_spring * unit_y;
    
    
                            node_x += f_spring_x;
                            node_y += f_spring_y;
                        }
    
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
    
        for i in 0..size {
            let mut node = nodes[i].write().unwrap();
            let node_next = nodes_next[i].read().unwrap();
            node.x = node_next.x;
            node.y = node_next.y;
        }


        let mut file = File::create(format!("result/{:04}.txt", epoch))?;
        for i in 0..size {
            let node = nodes[i].read().unwrap();
            // println!("{} {}", node.x, node.y);
            file.write_all(format!("{} {}\n", node.x, node.y).as_bytes())?;
        }
    }

    Ok(())

}
