mod graph;
mod model;
mod runner;
mod spring_model;
mod utils;

use std::sync::{Arc, RwLock};

use pyo3::exceptions;
use pyo3::{prelude::*, types::PyIterator};

#[pyfunction(number_of_nodes, edges, "*", iter = 500, threads = 0)]
fn layout_from_edge_list(
    number_of_nodes: usize,
    edges: &PyAny,
    iter: usize,
    threads: usize,
) -> PyResult<Vec<(f32, f32)>> {
    let model = Arc::new(RwLock::new(spring_model::SimpleSpringModel::new(1.0)));

    let mut edge_matrix = graph::new_edge_matrix(number_of_nodes);
    match edges.extract::<&PyIterator>() {
        Ok(iter) => {
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
    Ok(r.layout(number_of_nodes, edge_matrix, model))
}

#[pymodule]
fn graph_force(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(layout_from_edge_list, m)?)?;
    Ok(())
}
