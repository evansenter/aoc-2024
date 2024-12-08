use crate::utils::input;

const MAX_DELTA: i32 = 3;

pub fn solve() {
    let data = input::parse_int_grid(&input::read_file("inputs/day02.txt"));
    println!("Part 1: {}", solve_part1(&data));
}

fn solve_part1(data: &[Vec<i32>]) -> usize {
    data.iter()
        .filter(|row| monotonic_within_delta(row, MAX_DELTA))
        .count()
}

fn solve_part2(data: &[Vec<i32>]) -> usize {
    data.iter()
        .filter(|row| monotonic_within_delta(row, MAX_DELTA))
        .count()
}

fn monotonic_within_delta(data: &[i32], delta: i32) -> bool {
    // Early returns for invalid cases
    if data.len() < 2 {
        return false;
    }

    let first = data[0];
    let last = *data.last().unwrap();

    if first == last {
        return false;
    }

    let increasing = last > first;

    data.windows(2).all(|window| {
        let &[x, y] = window else {
            panic!("Window must contain exactly two elements");
        };

        (1..=delta).contains(&(y - x).abs()) && if increasing { y > x } else { y < x }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic_within_delta() {
        // Valid increasing sequences
        assert!(monotonic_within_delta(&[1, 2, 3], 3));
        assert!(monotonic_within_delta(&[1, 3, 4], 3));
        assert!(monotonic_within_delta(&[0, 2, 5], 3));

        // Valid decreasing sequences
        assert!(monotonic_within_delta(&[5, 3, 1], 3));
        assert!(monotonic_within_delta(&[6, 4, 2], 3));
        assert!(monotonic_within_delta(&[9, 7, 4], 3));

        // Invalid sequences - delta too large
        assert!(!monotonic_within_delta(&[1, 5, 7], 3));
        assert!(!monotonic_within_delta(&[9, 5, 3], 3));

        // Invalid sequences - wrong direction
        assert!(!monotonic_within_delta(&[1, 3, 2], 3));
        assert!(!monotonic_within_delta(&[5, 2, 3], 3));

        // Edge cases
        assert!(!monotonic_within_delta(&[1], 3)); // Too short
        assert!(!monotonic_within_delta(&[2, 2], 3)); // No change
        assert!(!monotonic_within_delta(&[1, 2, 2, 3], 3)); // Plateau
    }
}
