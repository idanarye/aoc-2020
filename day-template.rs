use aoc_runner_derive::*;

#[derive(Debug)]
pub struct RowData {
}

#[aoc_generator(day`)]
pub fn input_generator(input: &str) -> Vec<RowData> {
    input.lines().map(|line| {
    }).collect()
}

#[aoc(day`, part1)]
pub fn solve_part1(input: &[RowData]) -> usize {
}

// #[aoc(day`, part2)]
// pub fn solve_part2(input: &[RowData]) -> usize {
// }