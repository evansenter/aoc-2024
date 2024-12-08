use crate::utils::input;

const MAX_DELTA: usize = 3;

pub fn solve() {
    let data = input::parse_int_grid(&input::read_file("inputs/day02.txt"));
    println!("Part 1: {}", solve_part1(&data));
    println!("Part 2: {}", solve_part2(&data));
}

fn solve_part1(data: &[Vec<i32>]) -> usize {
    data.iter()
        .filter(|row| monotonic_within_delta(row) == row.len() as isize)
        .count()
}

fn solve_part2(data: &[Vec<i32>]) -> usize {
    data.iter()
        .filter(|row| match monotonic_within_delta(row) {
            -1 => false,
            n if n == row.len() as isize => true,
            i => {
                // Check if removing either element at the break point creates a valid sequence
                monotonic_within_delta_edit_dist_1(row, i as usize)
            }
        })
        .count()
}

fn monotonic_within_delta(data: &[i32]) -> isize {
    // Early returns for invalid cases
    if data.len() < 2 || data.first() == data.last() {
        return -1;
    }

    let increasing = data.last() > data.first();

    for (i, window) in data.windows(2).enumerate() {
        let &[x, y] = window else {
            unreachable!("Window must contain exactly two elements");
        };

        let diff = y - x;
        let in_range = (1..=MAX_DELTA).contains(&(diff.abs() as usize));
        let monotonic = if increasing { diff > 0 } else { diff < 0 };

        if !in_range || !monotonic {
            return i as isize;
        }
    }

    data.len() as isize
}

fn monotonic_within_delta_edit_dist_1(row: &[i32], i: usize) -> bool {
    // Helper function to filter out an element at a specific index
    let filter_out_index = |skip_i: usize| -> Vec<i32> {
        row.iter()
            .enumerate()
            .filter(|&(idx, _)| idx != skip_i)
            .map(|(_, &x)| x)
            .collect()
    };

    // Check if removing either element creates a valid sequence
    let check_without = |skip_i| {
        let filtered = filter_out_index(skip_i);
        monotonic_within_delta(&filtered) == filtered.len() as isize
    };

    check_without(i) || check_without(i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic_within_delta() {
        // Valid increasing sequences
        assert_eq!(monotonic_within_delta(&[1, 2, 3]), 3);
        assert_eq!(monotonic_within_delta(&[1, 3, 4]), 3);
        assert_eq!(monotonic_within_delta(&[0, 2, 5]), 3);

        // Valid decreasing sequences
        assert_eq!(monotonic_within_delta(&[5, 3, 1]), 3);
        assert_eq!(monotonic_within_delta(&[6, 4, 2]), 3);
        assert_eq!(monotonic_within_delta(&[9, 7, 4]), 3);

        // Invalid sequences - delta too large
        assert_eq!(monotonic_within_delta(&[1, 5, 7]), 0);
        assert_eq!(monotonic_within_delta(&[9, 5, 3]), 0);

        // Invalid sequences - wrong direction
        assert_eq!(monotonic_within_delta(&[1, 3, 2]), 1);
        assert_eq!(monotonic_within_delta(&[5, 2, 3]), 1);

        // Edge cases
        assert_eq!(monotonic_within_delta(&[1]), -1); // Too short
        assert_eq!(monotonic_within_delta(&[2, 2]), -1); // No change
        assert_eq!(monotonic_within_delta(&[1, 2, 2, 3]), 1); // Plateau
    }

    #[test]
    fn test_monotonic_within_delta_edit_dist_1() {
        // Valid sequences with one edit
        assert!(monotonic_within_delta_edit_dist_1(&[1, 2, 6, 4], 1));
        assert!(monotonic_within_delta_edit_dist_1(&[1, 5, 3, 4], 0));
        assert!(monotonic_within_delta_edit_dist_1(&[5, 3, 1, 2], 2));
        assert!(monotonic_within_delta_edit_dist_1(&[6, 4, 0, 2], 2));

        // Invalid sequences even with one edit
        assert!(!monotonic_within_delta_edit_dist_1(&[1, 5, 7, 6], 0));
        assert!(!monotonic_within_delta_edit_dist_1(&[9, 5, 3, 4], 0));
        assert!(!monotonic_within_delta_edit_dist_1(&[9, 3, 2, 4], 1));
    }
}
