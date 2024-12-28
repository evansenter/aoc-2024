use crate::utils::input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type TransitionMap = HashMap<i32, HashSet<i32>>;
type PageList = Vec<i32>;

#[derive(Debug)]
struct TransitionRules {
    allowed: TransitionMap,
    forbidden: TransitionMap,
}

impl TransitionRules {
    fn from_transitions(transitions: &[Vec<i32>]) -> Self {
        let mut allowed: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut forbidden: HashMap<i32, HashSet<i32>> = HashMap::new();

        for transition in transitions {
            let [from, to] = transition[..] else {
                unreachable!("Transition should have 2 elements");
            };
            allowed.entry(from).or_default().insert(to);
            forbidden.entry(to).or_default().insert(from);
        }

        Self { allowed, forbidden }
    }

    fn is_valid_transition(&self, from: &i32, to: &i32) -> bool {
        if self.forbidden.get(from).map_or(false, |f| f.contains(to)) {
            return false;
        }

        self.allowed.get(from).map_or_else(
            || panic!("no transition exists from {} to {}", from, to),
            |a| a.contains(to),
        )
    }
}

#[derive(Debug)]
struct PageLists {
    lists: Vec<PageList>,
}

impl PageLists {
    fn from_str(input: &str) -> Self {
        let lists = input
            .lines()
            .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
            .collect();
        Self { lists }
    }

    fn get_center_value(&self, index: usize) -> i32 {
        let list = &self.lists[index];
        list[list.len() / 2]
    }

    fn is_valid_list(&self, index: usize, rules: &TransitionRules) -> bool {
        let list = &self.lists[index];
        list.iter()
            .tuple_combinations()
            .all(|(a, b)| rules.is_valid_transition(a, b))
    }

    fn find_invalid_indices(&self, rules: &TransitionRules) -> HashSet<usize> {
        (0..self.lists.len())
            .filter(|&i| !self.is_valid_list(i, rules))
            .collect()
    }

    fn sum_centers(&self, indices: &[usize]) -> i32 {
        indices.iter().map(|&i| self.get_center_value(i)).sum()
    }

    fn try_fix_invalid_lists(&mut self, rules: &TransitionRules) {
        let invalid = self.find_invalid_indices(rules);
        for &i in &invalid {
            let list = &mut self.lists[i];
            for (j, k) in (0..list.len()).tuple_combinations() {
                if !rules.is_valid_transition(&list[j], &list[k]) {
                    list.swap(j, k);
                }
            }
        }
    }
}

pub fn solve() {
    let input = input::read_file("inputs/day05.txt");
    let [transitions, page_lists] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("Invalid input format");
    };

    let transitions: Vec<Vec<i32>> = transitions
        .lines()
        .map(|line| line.split('|').filter_map(|s| s.parse().ok()).collect())
        .collect();

    let mut page_lists = PageLists::from_str(page_lists);
    let rules = TransitionRules::from_transitions(&transitions);

    println!("Part 1: {}", solve_part1(&page_lists, &rules));
    println!("Part 2: {}", solve_part2(&mut page_lists, &rules));
}

fn solve_part1(page_lists: &PageLists, rules: &TransitionRules) -> i32 {
    let invalid = page_lists.find_invalid_indices(rules);
    let valid: Vec<_> = (0..page_lists.lists.len())
        .filter(|i| !invalid.contains(i))
        .collect();
    page_lists.sum_centers(&valid)
}

fn solve_part2(page_lists: &mut PageLists, rules: &TransitionRules) -> i32 {
    let invalid: Vec<_> = page_lists.find_invalid_indices(rules).into_iter().collect();
    page_lists.try_fix_invalid_lists(rules);
    page_lists.sum_centers(&invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_rules() {
        let transitions = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
        let rules = TransitionRules::from_transitions(&transitions);

        assert!(rules.is_valid_transition(&1, &2));
        assert!(rules.is_valid_transition(&1, &3));
        assert!(rules.is_valid_transition(&2, &3));
        assert!(!rules.is_valid_transition(&2, &1));
        assert!(!rules.is_valid_transition(&3, &1));
    }

    #[test]
    fn test_page_lists_validity() {
        let lists = PageLists {
            lists: vec![vec![1, 2, 3], vec![3, 2, 1]],
        };
        let rules = TransitionRules::from_transitions(&[vec![1, 2], vec![2, 3], vec![1, 3]]);

        assert!(lists.is_valid_list(0, &rules));
        assert!(!lists.is_valid_list(1, &rules));
    }
}
