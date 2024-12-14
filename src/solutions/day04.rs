use crate::utils::input;
use itertools::{chain, iproduct};
use std::ops::Add;
use std::ops::Mul;

const XMAS_TARGET: &str = "XMAS";

pub fn solve() {
    let data = input::read_file("inputs/day04.txt");
    println!("{}", data);
    println!("Part 1: {}", solve_part1(&data));
    println!("Part 2: {}", solve_part2(&data));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ICoords(isize, isize);

impl Add for ICoords {
    type Output = ICoords;

    fn add(self, other: ICoords) -> ICoords {
        ICoords(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<isize> for ICoords {
    type Output = ICoords;

    fn mul(self, other: isize) -> ICoords {
        ICoords(self.0 * other, self.1 * other)
    }
}

impl ICoords {
    fn is_in_bounds(&self, grid_size: ICoords) -> bool {
        (0..grid_size.0).contains(&self.0) && (0..grid_size.1).contains(&self.1)
    }
}

fn solve_part1(data: &str) -> usize {
    let lines: &Vec<Vec<char>> = &data
        .lines()
        .map(|line: &str| line.chars().collect())
        .collect();
    let grid_size: ICoords = ICoords(lines.len() as isize, lines[0].len() as isize);

    iproduct!(0..grid_size.0, 0..grid_size.1, -1..=1, -1..=1)
        .filter_map(|(row, col, row_dir, col_dir)| {
            if lines[row as usize][col as usize] != XMAS_TARGET.chars().nth(0).unwrap() {
                None
            } else {
                Some(
                    chain(
                        std::iter::once(ICoords(0, 0)),
                        (1..XMAS_TARGET.len()).map(|i| ICoords(row_dir, col_dir) * (i as isize)),
                    )
                    .map(|offset: ICoords| ICoords(row, col) + offset)
                    .collect(),
                )
            }
        })
        .filter(|trace: &Vec<ICoords>| {
            trace
                .last()
                .expect("trace should never be empty")
                .is_in_bounds(grid_size)
                && trace.first() != trace.last()
        })
        .map(|trace: Vec<ICoords>| {
            trace
                .into_iter()
                .map(|ICoords(row, col)| lines[row as usize][col as usize])
                .collect::<String>()
        })
        .filter(|s: &String| s == XMAS_TARGET)
        .count()
}

fn solve_part2(data: &str) -> i32 {
    0
}
