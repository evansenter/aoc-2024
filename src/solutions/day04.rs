use crate::utils::input;
use itertools::{iproduct, Itertools};
use std::ops::{Add, Mul};

const PATTERNS: [&str; 2] = ["XMAS", "MAS"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<isize> for Coord {
    type Output = Self;
    fn mul(self, scalar: isize) -> Self {
        Self(self.0 * scalar, self.1 * scalar)
    }
}

impl Coord {
    fn new(row: isize, col: isize) -> Self {
        Self(row, col)
    }

    fn in_bounds(&self, size: Self) -> bool {
        (0..size.0).contains(&self.0) && (0..size.1).contains(&self.1)
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
    size: Coord,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let size = Coord::new(cells.len() as isize, cells[0].len() as isize);
        Self { cells, size }
    }

    fn get(&self, coord: Coord) -> Option<char> {
        coord
            .in_bounds(self.size)
            .then(|| self.cells[coord.0 as usize][coord.1 as usize])
    }

    fn find_pattern(&self, pattern: &str, diagonal_only: bool) -> Vec<Vec<Coord>> {
        let direction: Vec<_> = if diagonal_only {
            iproduct!([-1, 1], [-1, 1]).collect()
        } else {
            iproduct!(-1..=1, -1..=1)
                .filter(|&(dr, dc)| dr != 0 || dc != 0)
                .collect()
        };
        let direction = direction.into_iter().map(|(dr, dc)| Coord::new(dr, dc));

        iproduct!(0..self.size.0, 0..self.size.1)
            .map(|(row, col)| Coord::new(row, col))
            .filter(|&start| self.get(start) == pattern.chars().next())
            .flat_map(|start| {
                direction.clone().filter_map(move |dir| {
                    // Draw a trace in the direction of the pattern
                    let trace: Vec<_> = (0..pattern.len())
                        .map(|i| start + dir * (i as isize))
                        .collect();

                    // Check if the trace is within bounds and matches the pattern
                    let valid = trace.last().unwrap().in_bounds(self.size)
                        && trace
                            .iter()
                            .map(|&pos| self.get(pos).expect("out of bounds"))
                            .collect::<String>()
                            == pattern;

                    valid.then_some(trace)
                })
            })
            .collect()
    }
}

pub fn solve() {
    let input = input::read_file("inputs/day04.txt");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    Grid::from_str(input).find_pattern(PATTERNS[0], false).len()
}

fn solve_part2(input: &str) -> usize {
    Grid::from_str(input)
        .find_pattern(PATTERNS[1], true)
        .into_iter()
        .filter_map(|trace| trace.get(1).copied())
        .counts()
        .values()
        .filter(|&&count| count > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_operations() {
        assert_eq!(Coord::new(1, 2) + Coord::new(3, 4), Coord::new(4, 6));
        assert_eq!(Coord::new(2, 3) * 2, Coord::new(4, 6));
    }

    #[test]
    fn test_coord_bounds() {
        let size = Coord::new(5, 5);
        assert!(Coord::new(0, 0).in_bounds(size));
        assert!(Coord::new(4, 4).in_bounds(size));
        assert!(!Coord::new(5, 5).in_bounds(size));
        assert!(!Coord::new(-1, 0).in_bounds(size));
    }

    #[test]
    fn test_grid_pattern_finding() {
        let input = "XMAS\nABCD\nXMAS\n";
        let grid = Grid::from_str(input);
        assert_eq!(grid.find_pattern("XM", false).len(), 2);
    }
}
