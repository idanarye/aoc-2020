use hashbrown::{HashSet, HashMap};

type Coord = [isize; 4];

#[derive(Debug, Clone)]
pub struct Grid(HashSet<Coord>);

impl Grid {
    fn min_max_coord(&self) -> Option<(Coord, Coord)> {
        let mut it = self.0.iter();
        let mut min = it.next()?.clone();
        let mut max = min.clone();
        for coord in it {
            for ((mn, mx), &val) in min.iter_mut().zip(max.iter_mut()).zip(coord) {
                if val < *mn {
                    *mn = val;
                }
                if *mx < val {
                    *mx = val;
                }
            }
        }
        Some((min, max))
    }
}

impl core::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = if let Some(minmax) = self.min_max_coord() {
            minmax
        } else {
            return Ok(());
        };
        let mut entries: Vec<_> = self.0.iter().collect();
        entries.sort_by_key(|[x, y, z, w]| (w, z, y, x));
        let mut it = entries.iter();
        let mut next_to_print = it.next();
        for w in min[3]..=max[3] {
            for z in min[2]..=max[2] {
                writeln!(f, "z={}, w={}", z, w)?;
                for y in min[1]..=max[1] {
                    for x in min[0]..=max[0] {
                        if Some(&&[x, y, z, w]) == next_to_print {
                            write!(f, "#")?;
                            next_to_print = it.next();
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    writeln!(f, "")?;
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

pub fn generator(input: &str) -> Grid {
    Grid(
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
            match c {
                '.' => None,
                '#' => Some([x as isize, y as isize, 0, 0]),
                _ => panic!("Illegal character {:?}", c),
            }
        })
    }).collect())
}

fn iter_neighbors(coord: Coord, added_ranges: &[std::ops::Range<isize>; 4]) -> impl Iterator<Item = Coord> {
    let added_ranges = added_ranges.clone();
    added_ranges[0].clone().flat_map(move |x| {
        let added_ranges = added_ranges.clone();
        added_ranges[1].clone().flat_map(move |y| {
            let added_ranges = added_ranges.clone();
            added_ranges[2].clone().flat_map(move |z| {
                added_ranges[3].clone().map(move |w| {
                    [coord[0] + x, coord[1] + y, coord[2] + z, coord[3] + w]
                })
            })
        })
    })
}

impl Grid {
    fn run_cycle(&self, added_ranges: &[std::ops::Range<isize>; 4]) -> Grid {
        let mut neighboring = HashMap::new();
        for coord in self.0.iter() {
            for neighbor in iter_neighbors(*coord, added_ranges) {
                if neighbor != *coord {
                    *neighboring.entry(neighbor).or_insert(0) += 1;
                }
            }
        }
        Grid(neighboring.iter().filter_map(|(coord, num_neighbors)| {
            match num_neighbors {
                2 => if self.0.contains(coord) {
                    Some(*coord)
                } else {
                    None
                },
                3 => Some(*coord),
                _ => None,
            }
        }).collect())
    }
}

pub fn part_1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for _ in 0..6 {
        grid = grid.run_cycle(&[-1..2, -1..2, -1..2, 0..1]);
    }
    grid.0.len()
}

pub fn part_2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for _ in 0..6 {
        grid = grid.run_cycle(&[-1..2, -1..2, -1..2, -1..2]);
    }
    grid.0.len()
}
