mod graph;
mod model;
mod runner;
mod spring_model;
mod utils;

use std::sync::{Arc, RwLock};

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction(number_of_nodes, edges, "*", iter = 500, threads = 0)]
fn layout_from_edge_list(
    number_of_nodes: usize,
    edges: Vec<(u32, u32)>,
    iter: usize,
    threads: usize,
) -> PyResult<Vec<(f32, f32)>> {
    let model = Arc::new(RwLock::new(spring_model::SimpleSpringModel::new(1.0)));
    let r = runner::Runner::new(iter, threads);
    Ok(r.layout(number_of_nodes, edges, model))
}

/// A Python module implemented in Rust.
#[pymodule]
fn graph_force(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(layout_from_edge_list, m)?)?;
    Ok(())
}
