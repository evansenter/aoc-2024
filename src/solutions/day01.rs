use crate::utils::{input, vec_tools};
use defaultmap::*;

pub fn solve() {
    let data = input::parse_int_grid(&input::read_file("inputs/day01.txt"));
    println!("Part 1: {}", solve_part1(&data));
    println!("Part 2: {}", solve_part2(&data));
}

fn solve_part1(data: &[Vec<i32>]) -> i32 {
    let sorted = sort_transposed(data.to_vec());
    let [a, b] = sorted.as_slice() else {
        panic!("Input must contain exactly two vectors");
    };
    a.iter().zip(b.iter()).map(|(a, b)| (b - a).abs()).sum()
}

fn solve_part2(data: &[Vec<i32>]) -> i32 {
    let data = vec_tools::transpose(&data.to_vec());
    let counts = data[1].iter().fold(
        defaulthashmap! {},
        |mut acc: DefaultHashMap<i32, i32>, &id| {
            acc[id] += 1;
            acc
        },
    );

    data[0].iter().map(|x| x * counts[x]).sum()
}

fn sort_transposed(data: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed: Vec<Vec<i32>> = vec_tools::transpose(&data);
    transposed
        .iter_mut()
        .for_each(|vec: &mut Vec<i32>| vec.sort());
    transposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_transposed_single_row() {
        let input = vec![vec![3, 1, 2]];
        let expected = vec![vec![3], vec![1], vec![2]];
        assert_eq!(sort_transposed(input), expected);
    }

    #[test]
    fn test_sort_transposed_single_column() {
        let input = vec![vec![3], vec![1], vec![2]];
        let expected = vec![vec![1, 2, 3]];
        assert_eq!(sort_transposed(input), expected);
    }

    #[test]
    fn test_sort_transposed() {
        let input = vec![vec![3, 5, 7], vec![1, 4, 9], vec![2, 6, 8]];
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(sort_transposed(input), expected);
    }
}
