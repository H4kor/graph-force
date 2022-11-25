mod graph;
mod model;
mod networkx_model;
mod reader;
mod runner;
mod spring_model;
mod utils;

use graph::NodeVector;
use pyo3::exceptions;
use pyo3::{prelude::*, types::PyIterator};

fn pick_model(model: &str) -> Result<Box<dyn model::ForceModel + Send + Sync>, PyErr> {
    match model {
        "spring_model" => Ok(Box::new(spring_model::SimpleSpringModel::new(1.0))),
        "networkx_model" => Ok(Box::new(networkx_model::NetworkXModel::new())),
        _ => {
            return Err(PyErr::new::<exceptions::PyValueError, _>(
                "model must be either 'spring_model' or 'networkx_model'",
            ))
        }
    }
}

fn initial_pos_to_node_vector(initial_pos: Option<Vec<(f32, f32)>>) -> Option<NodeVector> {
    match initial_pos {
        Some(pos) => {
            let nodes = graph::new_node_vector(pos.len());
            for (i, (x, y)) in pos.iter().enumerate() {
                let mut node = nodes[i].write().unwrap();
                node.x = *x;
                node.y = *y;
            }
            Some(nodes)
        }
        None => None,
    }
}

#[pyfunction(
    file_path,
    "*",
    iter = 500,
    threads = 0,
    model = "\"spring_model\"",
    initial_pos = "None"
)]
fn layout_from_edge_file(
    file_path: &str,
    iter: usize,
    threads: usize,
    model: &str,
    initial_pos: Option<Vec<(f32, f32)>>,
) -> PyResult<Vec<(f32, f32)>> {
    let (size, matrix) = reader::read_graph(file_path);
    let model = pick_model(model)?;

    let r = runner::Runner::new(iter, threads);
    Ok(r.layout(size, matrix, model, initial_pos_to_node_vector(initial_pos)))
}

#[pyfunction(
    number_of_nodes,
    edges,
    "*",
    iter = 500,
    threads = 0,
    model = "\"spring_model\"",
    initial_pos = "None"
)]
fn layout_from_edge_list(
    number_of_nodes: usize,
    edges: &PyAny,
    iter: usize,
    threads: usize,
    model: &str,
    initial_pos: Option<Vec<(f32, f32)>>,
) -> PyResult<Vec<(f32, f32)>> {
    let model: Box<dyn model::ForceModel + Send + Sync> = pick_model(model)?;

    let mut edge_matrix = graph::new_edge_matrix(number_of_nodes);
    match edges.extract::<&PyIterator>() {
        Ok(iter) => {
            iter.iter()?
                .map(|edge| edge.and_then(PyAny::extract::<(usize, usize)>))
                .for_each(|edge| {
                    if let Ok((u, v)) = edge {
                        graph::add_edge(&mut edge_matrix, u, v);
                    }
                });
            for edge in iter {
                let edge = edge?;
                let edge = edge.extract::<(usize, usize)>()?;
                graph::add_edge(&mut edge_matrix, edge.0, edge.1);
            }
        }
        Err(_) => match edges.extract::<Vec<(usize, usize)>>() {
            Ok(edge) => {
                for edge in edge {
                    graph::add_edge(&mut edge_matrix, edge.0, edge.1);
                }
            }
            Err(_) => {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(
                    "Edges must be an iterable of (int, int)",
                ));
            }
        },
    }

    let r = runner::Runner::new(iter, threads);
    Ok(r.layout(
        number_of_nodes,
        edge_matrix,
        model,
        initial_pos_to_node_vector(initial_pos),
    ))
}

#[pymodule]
fn graph_force(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(layout_from_edge_list, m)?)?;
    m.add_function(wrap_pyfunction!(layout_from_edge_file, m)?)?;
    Ok(())
}
