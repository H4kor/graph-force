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
