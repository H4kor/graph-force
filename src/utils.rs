use std::ops::Range;

pub fn gen_chunks(n: usize, chunks: usize) -> Vec<Range<usize>> {
    let mut borders = vec![];
    let chunk_size = n / chunks;
    let mut remainder = n % chunks;
    let mut start = 0;
    for _ in 0..chunks {
        let end = start + chunk_size;
        if remainder > 0 {
            remainder -= 1;
            borders.push(start..end + 1);
            start = end + 1;
        } else {
            borders.push(start..end);
            start = end;
        }
    }
    // Change the last chunk to include the remainder
    let size = borders.len();
    borders[size - 1].end = n;
    borders
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_chunks() {
        let borders = gen_chunks(10, 3);
        assert_eq!(borders, vec![(0..4), (4..7), (7..10)]);
    }

    #[test]
    fn test_gen_chunks2() {
        let borders = gen_chunks(10, 2);
        assert_eq!(borders, vec![(0..5), (5..10)]);
    }

    #[test]
    fn test_gen_chunks3() {
        let borders = gen_chunks(10, 1);
        assert_eq!(borders, vec![(0..10)]);
    }

    #[test]
    fn test_gen_chunks4() {
        let borders = gen_chunks(11, 3);
        assert_eq!(borders, vec![(0..4), (4..8), (8..11)]);
    }
}
