
use rand::Rng;
use std::sync::{Arc, RwLock};
use std::thread;
use crate::graph::{EdgeMatrix, NodeVector, Node, Edge};
use crate::my_model;
use crate::utils;


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

fn edge_matrix_from_edge_list(number_of_nodes: usize, edge_list: Vec<(u32, u32)>) -> EdgeMatrix {
    let matrix_ptr = connection_matrix(number_of_nodes as usize);
    {
        let mut matrix = matrix_ptr.write().unwrap();
        for (node_a, node_b) in edge_list {
            matrix[node_a as usize][node_b as usize].weight = 1.0;
            matrix[node_b as usize][node_a as usize].weight = 1.0;
        }
    }
    matrix_ptr
}

pub fn layout(number_of_nodes: usize, edge_list: Vec<(u32, u32)>) -> Vec<(f32, f32)> {
    const ITER: usize = 5000;
    const THREADS: usize = 8;

    // let edges = connection_matrix(size);
    let edges = edge_matrix_from_edge_list(number_of_nodes, edge_list);
    let mut nodes = nodes_list(number_of_nodes);
    let mut nodes_next = nodes_list(number_of_nodes);

    // let model = Arc::new(RwLock::new(spring_model::InitialModel::new(edges, number_of_nodes)));
    let model = Arc::new(RwLock::new(my_model::MyModel::new(edges, number_of_nodes, ITER)));

    let chunks = utils::gen_chunks(number_of_nodes, THREADS);
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
    }

    let mut result = vec![];
    for node in nodes.iter() {
        let node = node.read().unwrap();
        result.push((node.x, node.y));
    }
    result
}
