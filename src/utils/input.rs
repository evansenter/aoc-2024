use std::fs;

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read file: {}", filename))
}

pub fn parse_int_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .map(|s: &str| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_file("inputs/day01_test.txt")
    }

    #[test]
    fn test_parse_int_grid() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(parse_int_grid(input), expected);
    }

    #[test]
    fn test_parse_int_grid_single_line() {
        let input = "10 20 30";
        let expected = vec![vec![10, 20, 30]];
        assert_eq!(parse_int_grid(input), expected);
    }

    #[test]
    fn test_parse_data_with_actual_input() {
        let input = get_test_input();
        let result = parse_int_grid(&input);

        // Basic structure tests
        assert!(!result.is_empty());
        assert!(result.iter().all(|row| !row.is_empty()));
        assert!(result.iter().all(|row| row.iter().all(|&x| x >= 0)));
    }
}
