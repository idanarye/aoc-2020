use aoc_runner_derive::*;

use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

#[derive(Debug)]
pub struct RowData {
    outer: String,
    inner: HashMap<String, usize>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<RowData> {
    let outer_pattern = Regex::new(r"^(.*) bags? contain (.*)$").unwrap();
    let inner_pattern = Regex::new(r"(\d+) (.+?) bags?[,.]").unwrap();
    input.lines().map(|line| {
        let m = outer_pattern.captures(line).unwrap();
        RowData {
            outer: m[1].to_owned(),
            inner: inner_pattern
                .captures_iter(&m[2])
                .map(|m| (m[2].to_owned(), m[1].parse().unwrap()))
                .collect(),
        }
    }).collect()
}

struct Rules<'a>(HashMap<&'a str, &'a RowData>);

impl<'a> Rules<'a> {
    fn new(input: &'a [RowData]) -> Self {
        Self(
            input.iter()
            .map(|r| (r.outer.as_str(), r))
            .collect())
    }

    fn reversed(&self) -> HashMap<&'a str, HashSet<&'a str>> {
        let mut result: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
        for (outer, r) in self.0.iter() {
            for inner in r.inner.keys() {
                result.entry(inner.as_str()).or_default().insert(outer);
            }
        }
        result
    }

    fn count_inside_including(&self, outer: &str) -> usize {
        let mut result = 1;
        if let Some(row_data) = self.0.get(outer) {
            for (inner, number) in row_data.inner.iter() {
                result += number * self.count_inside_including(inner);
            }
        }
        result
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[RowData]) -> usize {
    let rules = Rules::new(input);
    let reversed = rules.reversed();

    let mut to_check = VecDeque::new();
    to_check.push_back("shiny gold");

    let mut contains_target_recursively = HashSet::new();

    while let Some(inner) = to_check.pop_front() {
        let is_new_entry = contains_target_recursively.insert(inner);
        if !is_new_entry {
            continue;
        }
        to_check.push_back(inner);
        if let Some(outers) = reversed.get(inner) {
            for outer in outers.iter() {
                to_check.push_back(outer);
            }
        }
    }
    contains_target_recursively.len() - 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[RowData]) -> usize {
    Rules::new(input).count_inside_including("shiny gold") - 1
}
