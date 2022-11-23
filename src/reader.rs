use std::{fs::File, io::Read};

use crate::graph::{add_edge, new_edge_matrix, EdgeMatrix};

/**
 * Read Graph data from file
 * Format:
 *  - Little endian
 *  - 4 bytes: number of nodes(int)
 *  - 12 bytes: nodeA(int), nodeB(int), weight(float)
 */
pub fn read_graph(file_name: &str) -> (usize, EdgeMatrix) {
    let mut file = File::open(file_name).expect("file not found");
    let mut size_buffer = [0; 4];
    file.read_exact(&mut size_buffer).expect("buffer overflow");
    let size = u32::from_le_bytes(size_buffer) as usize;
    let mut matrix = new_edge_matrix(size);
    {
        let mut buffer = [0; 12];
        while file.read_exact(&mut buffer).is_ok() {
            let node_a = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) as usize;
            let node_b = u32::from_le_bytes(buffer[4..8].try_into().unwrap()) as usize;
            let _weight = f32::from_le_bytes(buffer[8..12].try_into().unwrap());
            add_edge(&mut matrix, node_a, node_b);
        }
    }
    (size, matrix)
}
