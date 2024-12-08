use crate::utils::{input, vec_tools};

pub fn solve() {
    let input = input::read_file("inputs/day01.txt");
    let mut data: Vec<Vec<i32>> = parse_data(&input);
    data = sort_transposed(data);

    let deltas = data.iter().map(|vec: &Vec<i32>| (vec[1] - vec[0]).abs());

    println!("{:#?}", deltas.sum::<i32>());
}

fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .map(|s: &str| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn sort_transposed(mut data: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    data = vec_tools::transpose(&data);

    for vec in &mut data {
        vec.sort();
    }

    vec_tools::transpose(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        input::read_file("inputs/day01_test.txt")
    }

    #[test]
    fn test_parse_data() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(parse_data(input), expected);
    }

    #[test]
    fn test_parse_data_single_line() {
        let input = "10 20 30";
        let expected = vec![vec![10, 20, 30]];
        assert_eq!(parse_data(input), expected);
    }

    #[test]
    fn test_parse_data_with_actual_input() {
        let input = get_test_input();
        let result = parse_data(&input);

        // Basic structure tests
        assert!(!result.is_empty());
        assert!(result.iter().all(|row| !row.is_empty()));
        assert!(result.iter().all(|row| row.iter().all(|&x| x >= 0)));
    }

    #[test]
    fn test_sort_transposed_single_row() {
        let input = vec![vec![3, 1, 2]];
        let expected = vec![vec![3, 1, 2]];
        assert_eq!(sort_transposed(input), expected);
    }

    #[test]
    fn test_sort_transposed_single_column() {
        let input = vec![vec![3], vec![1], vec![2]];
        let expected = vec![vec![1], vec![2], vec![3]];
        assert_eq!(sort_transposed(input), expected);
    }

    #[test]
    fn test_sort_transposed() {
        let input = vec![vec![3, 5, 7], vec![1, 4, 9], vec![2, 6, 8]];
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(sort_transposed(input), expected);
    }
}
