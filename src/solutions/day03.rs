use crate::utils::input;
use regex::Regex;

const MULT_REGEX: &str = r"mul\((\d+),(\d+)\)";
const DONT_REGEX: &str = r"don't\(\)";
const DO_REGEX: &str = r"do\(\)";

pub fn solve() {
    let data = input::read_file("inputs/day03.txt");
    println!("Part 1: {}", solve_part1(&data));
    println!("Part 2: {}", solve_part2(&data));
}

fn solve_part1(data: &str) -> i32 {
    let re = Regex::new(MULT_REGEX).unwrap();

    re.captures_iter(data)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

fn solve_part2(data: &str) -> i32 {
    let dont_re = Regex::new(DONT_REGEX).unwrap();
    let do_re = Regex::new(DO_REGEX).unwrap();

    let dont_indices = dont_re
        .find_iter(data)
        .map(|m| m.start())
        .collect::<Vec<usize>>();
    
    let do_indices = do_re
        .find_iter(data)
        .map(|m| m.end())
        .collect::<Vec<usize>>();

    let mut dont_indices = dont_indices.iter().peekable();
    let mut do_indices = do_indices.iter().peekable();
    let mut rejection_spans = vec![];

    while let Some(&dont_start) = dont_indices.next() {
        // Skip all do instructions before the current dont instruction
        while do_indices.peek().is_some() && *do_indices.peek().unwrap() < &dont_start {
            do_indices.next();
        }

        match do_indices.next() {
            // If there's no matching 'do', reject until the end
            None => {
                rejection_spans.push(dont_start..data.len());
                break;
            },
            // Otherwise, reject until the next 'do'
            Some(&do_end) => {
                rejection_spans.push(dont_start..do_end);
                // Skip any dont instructions that fall within this rejection span
                while dont_indices.peek().map_or(false, |&&x| x < do_end) {
                    dont_indices.next();
                }
            }
        }
    }

    let mut data = data.to_string();
    // Remove spans in reverse order to maintain correct indices
    for span in rejection_spans.into_iter().rev() {
        data.replace_range(span, "");
    }

    solve_part1(&data)   
}
