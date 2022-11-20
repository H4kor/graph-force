mod layout;
mod utils;
mod spring_model;
mod my_model;
mod graph;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn layout_from_edge_list(number_of_nodes: usize, edges: Vec<(u32, u32)>) -> PyResult<Vec<(f32, f32)>> {
    Ok(
        layout::layout(number_of_nodes, edges)
    )
}

/// A Python module implemented in Rust.
#[pymodule]
fn graph_force(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(layout_from_edge_list, m)?)?;
    Ok(())
}