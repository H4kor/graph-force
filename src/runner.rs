use crate::graph::{new_edge_matrix, new_node_vector, EdgeMatrix};
use crate::model::ForceModel;
use crate::utils;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Runner {
    iterations: usize,
    threads: usize,
}

impl Runner {
    pub fn new(iterations: usize, threads: usize) -> Self {
        if threads == 0 {
            match std::thread::available_parallelism() {
                Ok(threads) => Runner {
                    iterations,
                    threads: threads.get(),
                },
                Err(_) => Runner {
                    iterations,
                    threads: 1,
                },
            }
        } else {
            Runner {
                iterations,
                threads,
            }
        }
    }

    pub fn layout<T: 'static + ForceModel + Sync + Send>(
        self: &Self,
        number_of_nodes: usize,
        edge_list: Vec<(u32, u32)>,
        model: Arc<RwLock<T>>,
    ) -> Vec<(f32, f32)> {
        // let edges = connection_matrix(size);
        let edges = edge_matrix_from_edge_list(number_of_nodes, edge_list);
        let mut nodes = new_node_vector(number_of_nodes);
        let mut nodes_next = new_node_vector(number_of_nodes);

        model
            .write()
            .unwrap()
            .init(edges, number_of_nodes, self.iterations);

        let chunks = utils::gen_chunks(number_of_nodes, self.threads);
        for _epoch in 0..self.iterations {
            model.write().unwrap().prepare(&nodes);
            let mut handles = vec![];
            for i in 0..self.threads {
                let nodes = nodes.clone();
                let nodes_next = nodes_next.clone();
                let chunk = chunks[i].clone();
                let model = model.clone();
                let handle = thread::spawn(move || {
                    for n in chunk {
                        let update = model.read().unwrap().step(&nodes, n);
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
}

fn edge_matrix_from_edge_list(number_of_nodes: usize, edge_list: Vec<(u32, u32)>) -> EdgeMatrix {
    let matrix_ptr = new_edge_matrix(number_of_nodes as usize);
    {
        let mut matrix = matrix_ptr.write().unwrap();
        for (node_a, node_b) in edge_list {
            matrix[node_a as usize][node_b as usize].weight = 1.0;
            matrix[node_b as usize][node_a as usize].weight = 1.0;
        }
    }
    matrix_ptr
}

#[cfg(test)]
mod test {
    use crate::graph::{Node, NodeVector};

    use super::*;

    struct MockModel {
        counter: usize,
    }

    impl ForceModel for MockModel {
        fn init(&mut self, _edges: EdgeMatrix, _size: usize, _iterations: usize) {}
        fn prepare(&mut self, _nodes: &NodeVector) {
            self.counter += 1;
        }
        fn step(&self, _nodes: &NodeVector, i_node: usize) -> Node {
            Node {
                x: i_node as f32,
                y: self.counter as f32,
            }
        }
    }

    #[test]
    fn test_edge_matrix_from_edge_list() {
        let edge_list = vec![(0, 1), (1, 2)];
        let matrix = edge_matrix_from_edge_list(3, edge_list);
        let matrix = matrix.read().unwrap();
        assert_eq!(matrix[0][1].weight, 1.0);
        assert_eq!(matrix[1][0].weight, 1.0);
        assert_eq!(matrix[1][2].weight, 1.0);
        assert_eq!(matrix[2][1].weight, 1.0);

        assert_eq!(matrix[0][0].weight, 0.0);
        assert_eq!(matrix[1][1].weight, 0.0);
        assert_eq!(matrix[2][2].weight, 0.0);

        assert_eq!(matrix[0][2].weight, 0.0);
        assert_eq!(matrix[2][0].weight, 0.0);
    }

    #[test]
    fn test_layout() {
        let edge_list = vec![];
        let model = Arc::new(RwLock::new(MockModel { counter: 0 }));
        let runner = Runner::new(10, 1);
        let result = runner.layout(3, edge_list, model);
        assert_eq!(result, vec![(0.0, 10.0), (1.0, 10.0), (2.0, 10.0)]);
    }
}
