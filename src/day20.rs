use core::fmt::Debug;
use core::ops::Index;

use hashbrown::{HashMap, HashSet};

const SIDE: usize = 10;

const SEA_MONSTER_PATTERN: &[&str] = &[
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

#[derive(Debug, Clone)]
pub struct Tile {
    bitmap: Vec<bool>,
}

type Input = HashMap<usize, Tile>;

pub fn generator(input: &str) -> Input {
    let header_pattern = regex::Regex::new(r"^Tile (\d+):$").unwrap();
    let mut it = input.lines().fuse();
    let mut result = HashMap::new();
    while let Some(header) = it.next() {
        let tile_id = header_pattern.captures(header).unwrap()[1].parse().unwrap();

        let bitmap = it.by_ref().take_while(|&line| line != "").flat_map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Illegal char {:?}", c),
                }
            })
        }).collect();

        result.insert(tile_id, Tile { bitmap });
    }
    result
}
impl Index<(usize, usize)> for Tile {
    type Output = bool;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(col < SIDE);
        &self.bitmap[row * SIDE + col]
    }
}

struct Row<'a> {
    tile: &'a Tile,
    row: usize,
}

struct Col<'a> {
    tile: &'a Tile,
    col: usize,
}

impl Tile {
    fn row(&self, row: usize) -> Row {
        Row {
            tile: &self,
            row,
        }
    }

    fn col(&self, col: usize) -> Col {
        Col {
            tile: &self,
            col,
        }
    }
}

impl Index<usize> for Row<'_> {
    type Output = bool;

    fn index(&self, col: usize) -> &Self::Output {
        &self.tile[(self.row, col)]
    }
}

impl Index<usize> for Col<'_> {
    type Output = bool;

    fn index(&self, row: usize) -> &Self::Output {
        &self.tile[(row, self.col)]
    }
}

impl Debug for Row<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str("Row[")?;
        for col in 0..SIDE {
            f.write_char(if self[col] { '#' } else { '.' })?;
        }
        f.write_char(']')?;
        Ok(())
    }
}

impl Debug for Col<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str("Col[")?;
        for row in 0..SIDE {
            f.write_char(if self[row] { '#' } else { '.' })?;
        }
        f.write_char(']')?;
        Ok(())
    }
}

fn get_code(obj: impl Index<usize, Output = bool>) -> u16 {
    let mut result = 0;
    for i in 0..SIDE {
        result <<= 1;
        if obj[i] {
            result |= 1;
        }
    }
    result
}

fn reverse_code(mut code: u16) -> u16 {
    let mut result = 0;
    for _ in 0..SIDE {
        result <<= 1;
        result |= code & 1;
        code >>= 1;
    }
    result
}

type CodeMap = HashMap<u16, HashSet<usize>>;

impl Tile {
    fn border_codes(&self) -> [u16; 4] {
        [
            get_code(self.row(0)),
            get_code(self.col(SIDE - 1)),
            reverse_code(get_code(self.row(SIDE - 1))),
            reverse_code(get_code(self.col(0))),
        ]
    }

    fn can_be_corner(&self, code_map: &CodeMap) -> bool {
        self.border_codes().iter().filter(|&&code| {
            1 < code_map[&reverse_code(code)].len()
        }).count() == 2
    }

    fn free_borders(&self, code_map: &CodeMap) -> Vec<usize> {
        self.border_codes().iter().enumerate().filter_map(|(idx, &code)| {
            if code_map[&reverse_code(code)].len() == 1 {
                Some(idx)
            } else {
                None
            }
        }).collect()
    }
}

fn prepare_code_map<'a>(tiles: &Input) -> CodeMap {
    let mut code_map = CodeMap::new();
    for (&tile_id, tile) in tiles {
        for &code in tile.border_codes().iter() {
            code_map.entry(code).or_insert_with(HashSet::new).insert(tile_id);
            let reversed = reverse_code(code);
            assert!(reversed != code);
            code_map.entry(reversed).or_insert_with(HashSet::new).insert(tile_id);
        }
    }
    code_map
}

pub fn part_1(tiles: &Input) -> usize {
    let code_map = prepare_code_map(tiles);
    tiles.iter().filter_map(|(tile_id, tile)| {
        if tile.can_be_corner(&code_map) {
            Some(tile_id)
        } else {
            None
        }
    }).product()
}

fn transpose_bitmap(bitmap: &mut [bool], side: usize) {
    for row in 0..side {
        for col in 0..row {
            bitmap.swap(row * side + col, col * side + row);
        }
    }
}

fn hflip_bitmap(bitmap: &mut [bool], side: usize) {
    for row in 0..side {
        for col in 0..(side / 2) {
            bitmap.swap(row * side + col, row * side + side - col - 1)
        }
    }
}

fn vflip_bitmap(bitmap: &mut [bool], side: usize) {
    for row in 0..(side / 2) {
        for col in 0..side {
            bitmap.swap(row * side + col, (side - row - 1) * side + col)
        }
    }
}

#[derive(Debug)]
enum BorderMatch {
    Straight(usize),
    Reversed(usize),
}

impl Tile {
    fn transposed(mut self) -> Tile {
        transpose_bitmap(self.bitmap.as_mut_slice(), SIDE);
        self
    }

    fn hflipped(mut self) -> Tile {
        hflip_bitmap(self.bitmap.as_mut_slice(), SIDE);
        self
    }

    fn vflipped(mut self) -> Tile {
        vflip_bitmap(self.bitmap.as_mut_slice(), SIDE);
        self
    }

    fn matching_border(&self, code_to_match: u16) -> BorderMatch {
        let reversed_to_match = reverse_code(code_to_match);
        self.border_codes().iter().enumerate().filter_map(|(i, &code)| {
            if code == code_to_match {
                Some(BorderMatch::Reversed(i))
            } else if code == reversed_to_match {
                Some(BorderMatch::Straight(i))
            } else {
                None
            }
        }).next().unwrap()
    }
}

fn organize_tiles(tiles: &Input) -> Vec<Vec<Tile>> {
    let code_map = prepare_code_map(tiles);
    let big_side = (tiles.len() as f64).sqrt() as usize;
    let mut result: Vec<Vec<Tile>> = Vec::new();
    let mut used_tiles = HashSet::new();

    let get_matching_tile = |code, prev_tile_id| {
        *code_map[&code].iter().filter(|&&tile_id| tile_id != prev_tile_id).next().unwrap()
    };

    let mut prev_row_start = None;
    for _ in 0..big_side {
        let mut current_row = Vec::new();
        let mut prev_tile_id = if let Some(prev_row_start) = prev_row_start {
            let code_to_match = reverse_code(get_code(result.last().unwrap()[0].row(SIDE - 1)));
            let tile_id = get_matching_tile(code_to_match, prev_row_start);
            let tile = tiles[&tile_id].clone();
            current_row.push(match tile.matching_border(code_to_match) {
                BorderMatch::Straight(0) => tile,
                BorderMatch::Straight(1) => tile.transposed().vflipped(),
                BorderMatch::Straight(2) => tile.vflipped().hflipped(),
                BorderMatch::Straight(3) => tile.transposed().hflipped(),
                BorderMatch::Reversed(0) => tile.hflipped(),
                BorderMatch::Reversed(1) => tile.transposed().hflipped().vflipped(),
                BorderMatch::Reversed(2) => tile.vflipped(),
                BorderMatch::Reversed(3) => tile.transposed(),
                ilg => panic!("Illegal {:?}", ilg),
            });
            tile_id
        } else {
            let (&corner_id, corner_tile) = tiles.iter().find(|(_, tile)| tile.can_be_corner(&code_map)).unwrap();
            used_tiles.insert(corner_id);

            let tile = match corner_tile.free_borders(&code_map).as_slice() {
                &[0, 1] => corner_tile.clone().transposed().vflipped(),
                &[1, 2] => corner_tile.clone().hflipped().vflipped(),
                &[2, 3] => corner_tile.clone().transposed().hflipped(),
                &[0, 3] => corner_tile.clone(),
                _ => panic!("")
            };
            current_row.push(tile);
            corner_id
        };
        prev_row_start = Some(prev_tile_id);

        for _ in 1..big_side {
            let code_to_match = get_code(current_row.last().unwrap().col(SIDE - 1));
            let tile_id = get_matching_tile(code_to_match, prev_tile_id);
            prev_tile_id = tile_id;
            let tile = tiles[&tile_id].clone();
            current_row.push(match tile.matching_border(code_to_match) {
                BorderMatch::Straight(0) => tile.transposed().vflipped(),
                BorderMatch::Straight(1) => tile.hflipped().vflipped(),
                BorderMatch::Straight(2) => tile.transposed().hflipped(),
                BorderMatch::Straight(3) => tile,
                BorderMatch::Reversed(0) => tile.transposed(),
                BorderMatch::Reversed(1) => tile.hflipped(),
                BorderMatch::Reversed(2) => tile.transposed().hflipped().vflipped(),
                BorderMatch::Reversed(3) => tile.vflipped(),
                ilg => panic!("Illegal {:?}", ilg),
            });
        }
        result.push(current_row);
    }

    result
}

fn squish_organized_tiles(organized: &Vec<Vec<Tile>>) -> Vec<bool> {
    let mut result = Vec::new();
    for tiles_row in organized.iter() {
        for subrow in 1..(SIDE - 1) {
            for tile in tiles_row.iter() {
                for subcol in 1..(SIDE - 1) {
                    result.push(tile[(subrow, subcol)]);
                }
            }
        }
    }
    result
}

#[derive(Debug)]
struct SeaMonsterPattern(Vec<(usize, usize)>);

impl SeaMonsterPattern {
    fn new(pattern: &[&str]) -> Self {
        Self(pattern.iter().enumerate().flat_map(|(row, row_data)| {
            row_data.chars().enumerate().filter_map(move |(col, c)| {
                if c == '#' {
                    Some((row, col))
                } else {
                    None
                }
            })
        }).collect())
    }

    fn is_at(&self, side: usize, map: &Vec<bool>, row: usize, col: usize) -> bool {
        for (r, c) in self.0.iter() {
            if !map[(row + r) * side + col + c] {
                return false;
            }
        }
        true
    }

    fn remove_at(&self, side: usize, map: &mut Vec<bool>, row: usize, col: usize) {
        for (r, c) in self.0.iter() {
            map[(row + r) * side + col + c] = false;
        }
    }

    fn find_all(&self, map: &Vec<bool>) -> Vec<(usize, usize)> {
        let side = (map.len() as f64).sqrt() as usize;
        let mut result = Vec::new();
        for row in 0..(side - SEA_MONSTER_PATTERN.len()) {
            for col in 0..(side - SEA_MONSTER_PATTERN[0].len()) {
                if self.is_at(side, map, row, col) {
                    result.push((row, col));
                }
            }
        }
        result
    }
}


pub fn part_2(tiles: &Input) -> usize {
    let organized = organize_tiles(tiles);

    let squished = squish_organized_tiles(&organized);
    let side = (squished.len() as f64).sqrt() as usize;

    let sea_monster_pattern = SeaMonsterPattern::new(SEA_MONSTER_PATTERN);

    let (mut map, monsters) = (0..0b1000).find_map(|choice| {
        let mut map = squished.clone();
        if 0 != (choice & 0b1) {
            transpose_bitmap(&mut map, side);
        }
        if 0 != (choice & 0b10) {
            hflip_bitmap(&mut map, side);
        }
        if 0 != (choice & 0b100) {
            vflip_bitmap(&mut map, side);
        }
        let monsters = sea_monster_pattern.find_all(&map);
        if monsters.is_empty() {
            None
        } else {
            Some((map, monsters))
        }
    }).unwrap();

    for (r, c) in monsters {
        sea_monster_pattern.remove_at(side, &mut map, r, c);
    }
    map.iter().filter(|&&value| value).count()
}
