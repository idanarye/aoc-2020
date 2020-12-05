use aoc_runner_derive::*;

#[derive(Debug)]
pub struct RowData {
    row: u8,
    column: u8,
}

impl RowData {
    pub fn new(txt: &str) -> Self {
        let mut it = txt.chars();

        let mut row = 0;
        for c in it.by_ref().take(7) {
            row <<= 1;
            match c {
                'F' => {
                },
                'B' => {
                    row |= 1;
                },
                _ => panic!("Illegal character {:?}", c),
            }
        }

        let mut column = 0;
        for c in it {
            column <<= 1;
            match c {
                'L' => {
                },
                'R' => {
                    column |= 1;
                },
                _ => panic!("Illegal character {:?}", c),
            }
        }

        RowData{ row, column }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<RowData> {
    input.lines().map(RowData::new).collect()
}

impl RowData {
    fn seat_id(&self) -> u16 {
        self.row as u16 * 8 + self.column as u16
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[RowData]) -> u16 {
    input.iter().map(|s| s.seat_id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[RowData]) -> u16 {
    let mut sorted = input.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|s| s.seat_id());
    for (seat, should_be) in sorted.iter().zip(sorted[0].seat_id()..) {
        if seat.seat_id() != should_be {
            return should_be;
        }
    }
    panic!("Could not find my missing seat");
}
