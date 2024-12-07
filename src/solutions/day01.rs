use crate::utils::input;

pub fn solve() {
    let input = input::read_file("inputs/day01.txt");

    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> u32 {
    // Your solution for part 1
    0
}

fn solve_part2(input: &str) -> u32 {
    // Your solution for part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "your test input here";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 0);
    }
}
