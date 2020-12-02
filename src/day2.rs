use aoc_runner_derive::*;

#[derive(Debug)]
pub struct RowData {
    range: std::ops::RangeInclusive<usize>,
    character: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<RowData> {
    let pattern = regex::Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    input.lines().map(|line| {
        let captures = pattern.captures(line).unwrap();
        RowData {
            range: captures.get(1).unwrap().as_str().parse().unwrap() ..= captures.get(2).unwrap().as_str().parse().unwrap(),
            character: captures.get(3).unwrap().as_str().chars().next().unwrap(),
            password: captures.get(4).unwrap().as_str().to_owned(),
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[RowData]) -> usize {
    input.iter().filter(|entry| {
        let num_occurences = entry.password.chars().filter(|&c| c == entry.character).count();
        entry.range.contains(&num_occurences)
    }).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[RowData]) -> usize {
    input.iter().filter(|entry| {
        let first_place_ok = entry.password.chars().skip(*entry.range.start() - 1).next() == Some(entry.character);
        let second_place_ok = entry.password.chars().skip(*entry.range.end() - 1).next() == Some(entry.character);
        first_place_ok != second_place_ok
    }).count()
}
