use crate::utils::input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = input::read_file("inputs/day05.txt");

    let [transitions, page_lists] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("Invalid input");
    };

    let transitions: Vec<Vec<i32>> = transitions
        .lines()
        .map(|line| line.split("|").filter_map(|s| s.parse().ok()).collect())
        .collect();

    let page_lists: Vec<Vec<i32>> = page_lists
        .lines()
        .map(|line| line.split(",").filter_map(|s| s.parse().ok()).collect())
        .collect();

    println!("{:?}", transitions);
    println!("{:?}", page_lists);

    println!("Part 1: {}", solve_part1(&transitions, &page_lists));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(transitions: &Vec<Vec<i32>>, page_lists: &[Vec<i32>]) -> i32 {
    let mut bad_transitions: HashMap<i32, HashSet<i32>> = HashMap::new();

    for transition in transitions {
        let [before, after] = transition[..] else {
            panic!("Invalid transition");
        };
        bad_transitions.entry(after).or_default().insert(before);
    }

    let mut invalid_page_list_indices: HashSet<usize> = HashSet::new();

    for (i, page_list) in page_lists.iter().enumerate() {
        page_list.iter().combinations(2).for_each(|v| {
            let [a, b] = v[..] else {
                unreachable!("Should never get here - combinations(2) always returns pairs")
            };

            if let Some(bad_transitions_for_a) = bad_transitions.get(a) {
                if bad_transitions_for_a.contains(b) {
                    invalid_page_list_indices.insert(i);
                }
            }
        });
    }

    page_lists
        .iter()
        .enumerate()
        .filter_map(|(i, page_list)| {
            if invalid_page_list_indices.contains(&i) {
                None
            } else {
                page_list.get((page_list.len() - 1) / 2)
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    0
}
