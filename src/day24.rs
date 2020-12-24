use hashbrown::{HashSet, HashMap};

#[derive(Debug)]
pub enum HexDirection {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug)]
pub struct Directions {
    hexdirs: Vec<HexDirection>,
}

pub fn generator(input: &str) -> Vec<Directions> {
    input.lines().map(|line| {
        let mut hexdirs = Vec::new();
        let mut it = line.chars();
        while let Some(c) = it.next() {
            hexdirs.push(
                match c {
                    'e' => HexDirection::E,
                    'w' => HexDirection::W,
                    's' => match it.next() {
                        Some('e') => HexDirection::SE,
                        Some('w') => HexDirection::SW,
                        Some(ilg) => panic!("Illegal characer {}", ilg),
                        None => panic!("No character after s"),
                    },
                    'n' => match it.next() {
                        Some('e') => HexDirection::NE,
                        Some('w') => HexDirection::NW,
                        Some(ilg) => panic!("Illegal characer {}", ilg),
                        None => panic!("No character after s"),
                    },
                    _ => panic!("Illegal characer {}", c),
                }
            );
        }
        Directions {hexdirs}
    }).collect()
}

type Coord = [isize; 2];

impl HexDirection {
    fn apply_to_coord(&self, [x, y]: Coord) -> Coord {
        match self {
            HexDirection::E => [x + 2, y],
            HexDirection::SE => [x + 1, y - 1],
            HexDirection::SW => [x - 1, y - 1],
            HexDirection::W => [x - 2, y],
            HexDirection::NW => [x - 1, y + 1],
            HexDirection::NE => [x + 1, y + 1],
        }
    }
}

impl Directions {
    fn resolve_from_origin_tile(&self, origin: Coord) -> Coord {
        self.hexdirs.iter().fold(origin, |coord, hd| hd.apply_to_coord(coord))
    }
}

pub fn black_tiles_from_input(input: &[Directions]) -> HashSet<Coord> {
    let mut black_tiles = HashMap::new();

    for directions in input.iter() {
        let tile_to_flip = directions.resolve_from_origin_tile([0 ,0]);
        match black_tiles.entry(tile_to_flip) {
            hashbrown::hash_map::Entry::Occupied(entry) => {
                entry.remove_entry();
            },
            hashbrown::hash_map::Entry::Vacant(entry) => {
                entry.insert(());
            },
        }
    }

    black_tiles.keys().copied().collect()
}

pub fn part_1(input: &[Directions]) -> usize {
    black_tiles_from_input(input).len()
}

impl HexDirection {
    fn all_directions() -> [Self; 6] {
        [
            Self::E,
            Self::SE,
            Self::SW,
            Self::W,
            Self::NW,
            Self::NE,
        ]
    }
}

pub fn part_2(input: &[Directions]) -> usize {
    let mut black_tiles = black_tiles_from_input(input);

    for _ in 0..100 {
        // The bool says if it already had a black tile before
        let mut neighbors_map: HashMap<Coord, (bool, usize)> = black_tiles.iter()
            .map(|&coord| (coord, (true, 0)))
            .collect();
        for &black_tile in black_tiles.iter() {
            for direction in HexDirection::all_directions().iter() {
                neighbors_map.entry(direction.apply_to_coord(black_tile)).or_insert((false, 0)).1 += 1;
            }
        }
        black_tiles = neighbors_map.into_iter().filter_map(|(coord, (was_black, num_neighbors))| {
            if was_black {
                if num_neighbors == 0 || 2 < num_neighbors {
                    None
                } else {
                    Some(coord)
                }
            } else {
                if num_neighbors == 2 {
                    Some(coord)
                } else {
                    None
                }
            }
        }).collect();
    }

    black_tiles.len()
}
