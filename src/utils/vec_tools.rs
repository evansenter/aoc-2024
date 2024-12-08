pub fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    (0..matrix[0].len())
        .map(|i| (0..matrix.len()).map(|j| matrix[j][i].clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_empty() {
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(transpose(&empty), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_transpose_single_element() {
        let matrix = vec![vec![1]];
        assert_eq!(transpose(&matrix), vec![vec![1]]);
    }

    #[test]
    fn test_transpose_square_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
        ];
        assert_eq!(transpose(&matrix), expected);
    }

    #[test]
    fn test_transpose_rectangular_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let expected = vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ];
        assert_eq!(transpose(&matrix), expected);
    }

    #[test]
    fn test_transpose_with_strings() {
        let matrix = vec![
            vec!["a", "b"],
            vec!["c", "d"],
        ];
        let expected = vec![
            vec!["a", "c"],
            vec!["b", "d"],
        ];
        assert_eq!(transpose(&matrix), expected);
    }
}
