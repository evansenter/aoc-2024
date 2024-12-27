use crate::utils::input;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn solve() {
    let input = input::read_file("inputs/day05_test.txt");

    let [transitions, page_lists] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("Invalid input");
    };

    let transitions: Vec<Vec<i32>> = transitions
        .lines()
        .map(|line| line.split("|").filter_map(|s| s.parse().ok()).collect())
        .collect();

    let mut page_lists: Vec<Vec<i32>> = page_lists
        .lines()
        .map(|line| line.split(",").filter_map(|s| s.parse().ok()).collect())
        .collect();

    println!("Part 1: {}", solve_part1(&transitions, &page_lists));
    println!("Part 2: {}", solve_part2(&transitions, page_lists.as_mut_slice()));
}

fn build_transition_maps(
    transitions: &[Vec<i32>],
) -> (HashMap<i32, HashSet<i32>>, HashMap<i32, HashSet<i32>>) {
    let mut forward: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut backward: HashMap<i32, HashSet<i32>> = HashMap::new();

    for transition in transitions {
        let [before, after] = transition[..] else {
            panic!("Invalid transition");
        };
        forward.entry(before).or_default().insert(after);
        backward.entry(after).or_default().insert(before);
    }

    (forward, backward)
}

fn sum_page_list_centers(
    page_lists: &[Vec<i32>],
    invalid_page_list_indices: &HashSet<usize>,
) -> i32 {
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

fn solve_part1(transitions: &Vec<Vec<i32>>, page_lists: &[Vec<i32>]) -> i32 {
    let (good_transitions, bad_transitions) = build_transition_maps(&transitions);

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
            } else if !good_transitions.get(a).unwrap().contains(b) {
                unreachable!("{} to {} not found", a, b);
            }
        });
    }

    log::debug!("Invalid page list indices: {:?}", invalid_page_list_indices);

    sum_page_list_centers(page_lists, &invalid_page_list_indices)
}

fn solve_part2(transitions: &Vec<Vec<i32>>, page_lists: &mut [Vec<i32>]) -> i32 {
    let (good_transitions, bad_transitions) = build_transition_maps(&transitions);

    let mut invalid_pages_by_prefix_by_index: HashMap<usize, BTreeMap<i32, Vec<i32>>> =
        HashMap::new();

    for (i, page_list) in page_lists.iter().enumerate() {
        let mut printed_header: bool = false;

        page_list.iter().combinations(2).for_each(|v| {
            let [a, b] = v[..] else {
                unreachable!("Should never get here - combinations(2) always returns pairs")
            };

            if let Some(bad_transitions_for_a) = bad_transitions.get(a) {
                if bad_transitions_for_a.contains(b) {
                    invalid_pages_by_prefix_by_index
                        .entry(i)
                        .or_default()
                        .entry(*a)
                        .or_default()
                        .push(*b);

                    if !printed_header {
                        log::debug!("Invalid page list: {:?}", page_list);
                        log::debug!("Good transitions: {:?}", good_transitions);
                        log::debug!("Bad transitions: {:?}", bad_transitions);
                        printed_header = true;
                    }

                    log::debug!("{} to {} is invalid", a, b);
                }
            } else if !good_transitions.get(a).unwrap().contains(b) {
                unreachable!("{} to {} not found", a, b);
            }
        });
    }

    log::debug!("Invalid page transitions: {:?}", invalid_pages_by_prefix_by_index);
    
    for (i, invalid_pages_by_prefix) in invalid_pages_by_prefix_by_index.into_iter() {
        let page_list = &mut page_lists[i];

        log::debug!("Page list {} before update: {:?}", i, page_list);

        invalid_pages_by_prefix.into_iter().rev().for_each(|(prefix, invalid_pages)| {
            log::debug!("Prefix: {:?}, invalid pages: {:?}", prefix, invalid_pages);

            let prefix_index = page_list.iter().position(|&page| page == prefix).unwrap();
            log::debug!("Prefix index: {:?}", prefix_index);

            let Some(max_invalid_page_index) = invalid_pages.iter().map(|&invalid_page| {
                page_list.iter().position(|&page| page == invalid_page).unwrap()
            }).max() else {
                unreachable!("No invalid pages found");
            };

            log::debug!("Max invalid page: {:?}", max_invalid_page_index);

            page_list.remove(prefix_index);
            page_list.insert(max_invalid_page_index, prefix);

            log::debug!("Page list {} after update: {:?}", i, page_list);
        });
    }

    solve_part1(transitions, page_lists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_transition_maps() {
        let transitions = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
        let (forward, backward) = build_transition_maps(&transitions);

        assert_eq!(forward[&1], [2, 3].into_iter().collect());
        assert_eq!(forward[&2], [3].into_iter().collect());
        assert_eq!(backward[&2], [1].into_iter().collect());
        assert_eq!(backward[&3], [1, 2].into_iter().collect());
    }
}
