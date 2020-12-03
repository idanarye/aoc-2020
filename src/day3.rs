use aoc_runner_derive::*;

type RowData = Vec<bool>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<RowData> {
    input.lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("WTF is {}", c),
        }).collect()
    }).collect()
}

fn calc_slope_trees(map: &[RowData], right: usize, down: usize) -> usize {
    map.iter().enumerate().filter(|(row_index, row_trees)| {
        if row_index % down == 0 {
            row_trees[(row_index / down * right) % row_trees.len()]
        } else {
            false
        }
    }).count()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[RowData]) -> usize {
    calc_slope_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[RowData]) -> usize {
    [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ].iter()
        .map(|&(right, down)| calc_slope_trees(input, right, down))
        .product()
}
