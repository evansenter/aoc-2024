use crate::utils::{input, vec_tools};
use defaultmap::*;

pub fn solve() {
    let input = input::read_file("inputs/day01.txt");
    let data: Vec<Vec<i32>> = input::parse_int_grid(&input);
    println!("Part 1: {:#?}", solve_part1(&data));
    println!("Part 2: {:#?}", solve_part2(&data));
}

fn solve_part1(data: &[Vec<i32>]) -> i32 {
    let data = sort_transposed(data.to_vec());
    data.iter()
        .map(|vec: &Vec<i32>| (vec[1] - vec[0]).abs())
        .sum()
}

fn solve_part2(data: &[Vec<i32>]) -> i32 {
    let data = vec_tools::transpose(&data.to_vec());
    let mut counts: DefaultHashMap<i32, i32> = defaulthashmap!{};

    for id in data[1].clone().into_iter() {
        counts[id] += 1;
    }

    data[0].iter().fold(0, |acc, &x| acc + x * counts[x])
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
