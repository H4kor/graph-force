mod utils;
mod spring_model;
mod my_model;
mod graph;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};
use std::thread;
use graph::{EdgeMatrix, NodeVector, Node, Edge};


fn nodes_list(size: usize) -> NodeVector {
    let mut nodes = Vec::new();
    for _ in 0..size {
        let node = RwLock::new(Node {
            x: rand::thread_rng().gen_range(-0.5..0.5),
            y: rand::thread_rng().gen_range(-0.5..0.5),
        });
        nodes.push(node);
    }
    Arc::new(nodes)
}

fn connection_matrix(size: usize) -> EdgeMatrix {
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

/**
 * Read Graph data from file
 * Format:
 *  - Little endian
 *  - 4 bytes: number of nodes(int)
 *  - 12 bytes: nodeA(int), nodeB(int), weight(float)
 */
fn read_graph(file_name: &str) -> (usize, EdgeMatrix) {
    let mut file = File::open(file_name).expect("file not found");
    let mut size_buffer = [0; 4];
    file.read_exact(&mut size_buffer).expect("buffer overflow");
    let size = u32::from_le_bytes(size_buffer) as usize;
    let matrix_ptr = connection_matrix(size);
    {
        let mut matrix = matrix_ptr.write().unwrap();
        let mut buffer = [0; 12];
        while file.read_exact(&mut buffer).is_ok() {
            let node_a = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) as usize;
            let node_b = u32::from_le_bytes(buffer[4..8].try_into().unwrap()) as usize;
            let weight = f32::from_le_bytes(buffer[8..12].try_into().unwrap());
            matrix[node_a][node_b].weight = weight;
            matrix[node_b][node_a].weight = weight;
        }
    }
    (size, matrix_ptr)
}

fn main() -> std::io::Result<()> {
    const ITER: usize = 50;
    const THREADS: usize = 8;

    // let edges = connection_matrix(size);
    let (size, edges): (usize, EdgeMatrix) = read_graph("../graph.bin");
    println!("Size: {}", size);
    let mut nodes = nodes_list(size);
    let mut nodes_next = nodes_list(size);

    // let model = Arc::new(RwLock::new(spring_model::InitialModel::new(edges, size)));
    let model = Arc::new(RwLock::new(my_model::MyModel::new(edges, size, ITER)));

    let chunks = utils::gen_chunks(size, THREADS);
    for epoch in 0..ITER {
        model.write().unwrap().prepare(&nodes);
        let mut handles = vec![];
        for i in 0..THREADS {
            let nodes = nodes.clone();
            let nodes_next = nodes_next.clone();
            let chunk = chunks[i].clone();
            let model = model.clone();
            let handle = thread::spawn(move || {
                for n in chunk {
                    let update = model.read().unwrap().step(&nodes,n);
                    let mut result = nodes_next[n].write().unwrap();
                    result.x = update.x;
                    result.y = update.y;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    
        // swap nodes and nodes_next
        let tmp = nodes.clone();
        nodes = nodes_next.clone();
        nodes_next = tmp.clone();
        
        let mut file = File::create(format!("result/{:04}.txt", epoch))?;
        for i in 0..size {
            let node = nodes[i].read().unwrap();
            // println!("{} {}", node.x, node.y);
            file.write_all(format!("{} {}\n", node.x, node.y).as_bytes())?;
        }
    }

    Ok(())
}
