#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

type Direction = [isize; 2];

const DIRECTIONS: [Direction; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

impl From<char> for SeatStatus {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Invalid seat code {:?}", c),
        }
    }
}

impl Into<char> for SeatStatus {
    fn into(self) -> char {
        match self {
            SeatStatus::Floor => '.',
            SeatStatus::Empty => 'L',
            SeatStatus::Occupied => '#',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    seats: Vec<Vec<SeatStatus>>,
}

impl core::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use core::fmt::Write;
        for (i, row) in self.seats.iter().enumerate() {
            if 0 < i {
                f.write_char('\n')?;
            }
            for &seat in row.iter() {
                f.write_char(seat.into())?;
            }
        }
        Ok(())
    }
}

pub fn generator(input: &str) -> Area {
    Area {
        seats: input.lines().map(|line| {
            line.chars().map(|c| c.into()).collect()
        }).collect()
    }
}

#[derive(Clone)]
struct SeatPos<'a> {
    area: &'a Area,
    row: usize,
    col: usize,
}

impl core::fmt::Debug for SeatPos<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SeatPos[{}, {}]", self.row, self.col)
    }
}

impl SeatPos<'_> {
    fn offset(&self, [row_offset, col_offset]: Direction) -> Option<Self> {
        let new_row = self.row as isize + row_offset;
        if !(0..self.area.num_rows() as isize).contains(&new_row) {
            return None
        }

        let new_col = self.col as isize + col_offset;
        if !(0..self.area.num_cols() as isize).contains(&new_col) {
            return None
        }

        Some(Self {
            area: self.area,
            row: new_row as usize,
            col: new_col as usize,
        })
    }

    fn visible_at_direction(&self, direction: Direction) -> Option<Self> {
        let mut other = self.clone();
        while let Some(seat) = other.offset(direction) {
            if seat.status() != SeatStatus::Floor {
                return Some(seat);
            }
            other = seat;
        }
        None
    }

    fn status(&self) -> SeatStatus {
        self.area.seats[self.row][self.col]
    }

    fn num_occupied_adjacent(&self) -> usize {
        let mut result = 0;
        for &direction in DIRECTIONS.iter() {
            if let Some(adj) = self.offset(direction){
                if adj.status() == SeatStatus::Occupied {
                    result += 1;
                }
            }
        }
        result
    }

    fn num_occupied_visible(&self) -> usize {
        let mut result = 0;
        for &direction in DIRECTIONS.iter() {
            if let Some(adj) = self.visible_at_direction(direction){
                if adj.status() == SeatStatus::Occupied {
                    result += 1;
                }
            }
        }
        result
    }
}

impl Area {
    fn num_rows(&self) -> usize {
        self.seats.len()
    }

    fn num_cols(&self) -> usize {
        self.seats[0].len()
    }

    fn iter_positions_by_row<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = SeatPos<'a>>> {
        (0..self.num_rows()).map(move |row| {
            (0..self.num_cols()).map(move |col| SeatPos {
                area: self,
                row,
                col,
            })
        })
    }

    fn advance(&self, rule: impl Fn(SeatPos) -> SeatStatus) -> Self {
        Self {
            seats: self.iter_positions_by_row().map(move |row_positions| {
                row_positions.map(&rule).collect()
            }).collect()
        }
    }

    fn count_occupied(&self) -> usize {
        self.iter_positions_by_row().flatten().filter(|pos| pos.status() == SeatStatus::Occupied).count()
    }

    fn advance_to_halt(self, rule: impl Fn(SeatPos) -> SeatStatus) -> Self {
        let mut area = self;
        loop {
            let next_area = area.advance(&rule);
            if area == next_area {
                break;
            }
            area = next_area;
        }
        area
    }
}

pub fn part_1(area: &Area) -> usize {
    area.clone().advance_to_halt(|seat_pos| {
        match seat_pos.status() {
            SeatStatus::Floor => SeatStatus::Floor,
            SeatStatus::Empty => {
                if seat_pos.num_occupied_adjacent() == 0 {
                    SeatStatus::Occupied
                } else {
                    SeatStatus::Empty
                }
            }
            SeatStatus::Occupied => {
                if 4 <= seat_pos.num_occupied_adjacent() {
                    SeatStatus::Empty
                } else {
                    SeatStatus::Occupied
                }
            }
        }
    }).count_occupied()
}

pub fn part_2(area: &Area) -> usize {
    area.clone().advance_to_halt(|seat_pos| {
        match seat_pos.status() {
            SeatStatus::Floor => SeatStatus::Floor,
            SeatStatus::Empty => {
                if seat_pos.num_occupied_visible() == 0 {
                    SeatStatus::Occupied
                } else {
                    SeatStatus::Empty
                }
            }
            SeatStatus::Occupied => {
                if 5 <= seat_pos.num_occupied_visible() {
                    SeatStatus::Empty
                } else {
                    SeatStatus::Occupied
                }
            }
        }
    }).count_occupied()
}
