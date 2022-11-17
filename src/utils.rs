use std::ops::Range;

pub fn chunk_borders(n: usize, chunks: usize) -> Vec<Range<usize>> {
    let mut borders = vec![];
    let chunk_size = n / chunks;
    let mut start = 0;
    for _ in 0..chunks {
        let end = start + chunk_size;
        borders.push(start..end);
        start = end;
    }
    // Change the last chunk to include the remainder
    let size = borders.len();
    borders[size - 1].end = n;
    borders
}


mod test {
    use super::*;

    #[test]
    fn test_chunk_borders() {
        let borders = chunk_borders(10, 3);
        assert_eq!(borders, vec![(0..3), (3..6), (6..10)]);
    }

    #[test]
    fn test_chunk_borders2() {
        let borders = chunk_borders(10, 2);
        assert_eq!(borders, vec![(0..5), (5..10)]);
    }

    #[test]
    fn test_chunk_borders3() {
        let borders = chunk_borders(10, 1);
        assert_eq!(borders, vec![(0..10)]);
    }

}
