use crate::utils::input;
use itertools::iproduct;
use std::ops::{Add, Mul};

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

pub fn solve() {
    let input = input::read_file("inputs/day04.txt");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        let size = Coord(cells.len() as isize, cells[0].len() as isize);
        Self { cells, size }
    }

    fn get(&self, coord: Coord) -> Option<char> {
        if coord.in_bounds(self.size) {
            Some(self.cells[coord.0 as usize][coord.1 as usize])
        } else {
            None
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let directions = iproduct!(-1..=1, -1..=1)
        .filter(|&(dr, dc)| dr != 0 || dc != 0)
        .map(|(dr, dc)| Coord(dr, dc));
    let grid_ref = &grid;

    iproduct!(0..grid.size.0, 0..grid.size.1)
        .map(|(row, col)| Coord(row, col))
        .filter(|&start| grid_ref.get(start) == Some(XMAS.chars().next().unwrap()))
        .flat_map(|start| {
            directions.clone().filter_map(move |dir| {
                let trace: Vec<_> = (0..XMAS.len())
                    .map(|i| start + dir * (i as isize))
                    .collect();

                let valid = trace.last().unwrap().in_bounds(grid_ref.size)
                    && trace
                        .iter()
                        .map(|&pos| grid_ref.get(pos))
                        .collect::<Option<String>>()
                        == Some(XMAS.to_string());

                valid.then_some(trace)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_operations() {
        assert_eq!(Coord(1, 2) + Coord(3, 4), Coord(4, 6));
        assert_eq!(Coord(2, 3) * 2, Coord(4, 6));
    }

    #[test]
    fn test_coord_bounds() {
        let size = Coord(5, 5);
        assert!(Coord(0, 0).in_bounds(size));
        assert!(Coord(4, 4).in_bounds(size));
        assert!(!Coord(5, 5).in_bounds(size));
        assert!(!Coord(-1, 0).in_bounds(size));
    }
}
