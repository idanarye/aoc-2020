type RowData = Vec<bool>;

pub fn generator(input: &str) -> Vec<RowData> {
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

pub fn part_1(input: &[RowData]) -> usize {
    calc_slope_trees(input, 3, 1)
}

pub fn part_2(input: &[RowData]) -> usize {
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
