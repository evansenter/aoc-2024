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

    let mut page_lists: Vec<Vec<i32>> = page_lists
        .lines()
        .map(|line| line.split(",").filter_map(|s| s.parse().ok()).collect())
        .collect();

    println!("Part 1: {}", solve_part1(&transitions, &page_lists));
    println!(
        "Part 2: {}",
        solve_part2(&transitions, page_lists.as_mut_slice())
    );
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

fn invalid_page_list_indices(
    page_lists: &[Vec<i32>],
    good_transitions: &HashMap<i32, HashSet<i32>>,
    bad_transitions: &HashMap<i32, HashSet<i32>>,
) -> HashSet<usize> {
    page_lists
        .iter()
        .enumerate()
        .filter_map(|(i, page_list)| {
            if !check_page_list_validity(page_list, good_transitions, bad_transitions) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn check_page_list_validity(
    page_list: &[i32],
    good_transitions: &HashMap<i32, HashSet<i32>>,
    bad_transitions: &HashMap<i32, HashSet<i32>>,
) -> bool {
    !page_list.iter().combinations(2).any(|v| {
        let [a, b] = v[..] else { unreachable!() };

        if let Some(bad_transitions_for_a) = bad_transitions.get(a) {
            bad_transitions_for_a.contains(b)
        } else if good_transitions.get(a).unwrap().contains(b) {
            false
        } else {
            unreachable!("{} to {} not found", a, b)
        }
    })
}

fn sum_page_list_centers(page_lists: &[Vec<i32>], pl_indices: &[usize]) -> i32 {
    pl_indices.iter().fold(0, |acc, i| {
        acc + page_lists[*i][(page_lists[*i].len() - 1) / 2]
    })
}

fn solve_part1(transitions: &[Vec<i32>], page_lists: &[Vec<i32>]) -> i32 {
    let (good_transitions, bad_transitions) = build_transition_maps(&transitions);

    let invalid_pl_indices =
        invalid_page_list_indices(page_lists, &good_transitions, &bad_transitions);

    let valid_pl_indices = (0..page_lists.len())
        .collect::<HashSet<usize>>()
        .difference(&invalid_pl_indices)
        .cloned()
        .collect::<Vec<usize>>();

    sum_page_list_centers(page_lists, &valid_pl_indices)
}

fn solve_part2(transitions: &[Vec<i32>], page_lists: &mut [Vec<i32>]) -> i32 {
    let (good_transitions, bad_transitions) = build_transition_maps(transitions);

    let invalid_pl_indices =
        invalid_page_list_indices(page_lists, &good_transitions, &bad_transitions);

    for i in invalid_pl_indices.iter() {
        let page_list = &mut page_lists[*i];

        for v in (0..page_list.len()).combinations(2) {
            let [j, k] = v[..] else { unreachable!() };

            if let Some(bad_transitions_for_a) = bad_transitions.get(&page_list[j]) {
                if bad_transitions_for_a.contains(&page_list[k]) {
                    page_list.swap(j, k);
                }
            }
        }
    }

    sum_page_list_centers(
        page_lists,
        &invalid_pl_indices.into_iter().collect::<Vec<_>>(),
    )
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
