use aoc_runner_derive::*;

use std::collections::HashSet;

#[derive(Debug)]
pub struct GroupData {
    rows: Vec<String>,
}

impl GroupData {
    fn new(rows: &[&str]) -> Self {
        Self {
            rows: rows.iter().map(|&r| r.to_owned()).collect(),
        }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<GroupData> {
    let mut result = Vec::new();
    let mut new_group = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            if !new_group.is_empty() {
                result.push(GroupData::new(&new_group));
                new_group = Vec::new();
            }
        } else {
            new_group.push(line);
        }
    }
    if !new_group.is_empty() {
        result.push(GroupData::new(&new_group));
    }
    result
}

impl GroupData {
    fn everyone_answered_yes(&self) -> HashSet<char> {
        self.rows.iter().flat_map(|row| row.chars()).collect()
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[GroupData]) -> usize {
    input.iter().map(|g| g.everyone_answered_yes().len()).sum()
}

impl GroupData {
    fn anyone_answered_yes(&self) -> HashSet<char> {
        let mut it = self.rows.iter().map(|row| row.chars().collect::<HashSet<char>>());
        let mut result = it.next().unwrap();
        for ans in it {
            result.retain(|a| ans.contains(a));
        }
        result
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[GroupData]) -> usize {
    input.iter().map(|g| g.anyone_answered_yes().len()).sum()
}
