#[derive(Debug, Copy, Clone)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Into<isize> for Direction {
    fn into(self) -> isize {
        match self {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
    }
}

impl From<isize> for Direction {
    fn from(direction: isize) -> Self {
        match direction.rem_euclid(4) {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            rem => panic!("rem_euclid(4) cannot result in {}", rem),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    MoveInDirection(Direction, isize),
    Rotate(isize),
    Forward(isize),
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let mut chars = line.chars();
        let instruction_type = chars.next().unwrap();
        let instruction_arg = chars.as_str().parse::<isize>().unwrap();
        match instruction_type {
            'F' => Instruction::Forward(instruction_arg),
            'R' => Instruction::Rotate(instruction_arg / 90),
            'L' => Instruction::Rotate(-instruction_arg / 90),
            _ => Instruction::MoveInDirection(match instruction_type{
                'N' => Direction::North,
                'S' => Direction::South,
                'E' => Direction::East,
                'W' => Direction::West,
                _ => panic!("Unknown instruction {:?}", instruction_type),
            }, instruction_arg)
        }
    }).collect()
}

type Position = [isize; 2];

impl Direction {
    fn move_position(&self, &[lat, long]: &Position, steps: isize) -> Position {
        match self {
            Direction::East => [lat, long + steps],
            Direction::South => [lat - steps, long],
            Direction::West => [lat, long - steps],
            Direction::North => [lat + steps, long],
        }
    }
}

#[derive(Debug)]
struct ShipState {
    position: Position,
    direction: Direction,
}

impl ShipState {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::MoveInDirection(direction, steps) => {
                self.position = direction.move_position(&self.position, *steps);
            }
            Instruction::Rotate(num_turns) => {
                self.direction = (Into::<isize>::into(self.direction) + *num_turns).into();
            }
            Instruction::Forward(steps) => {
                self.position = self.direction.move_position(&self.position, *steps);
            }
        }
    }

    fn manhatten_distance(&self) -> usize {
        self.position.iter().map(|coord| coord.abs() as usize).sum()
    }
}

pub fn part_1(input: &[Instruction]) -> usize {
    let mut ship_state = ShipState {
        position: [0, 0],
        direction: Direction::East,
    };
    for instruction in input.iter() {
        ship_state.apply_instruction(instruction);
    }
    ship_state.manhatten_distance()
}

#[derive(Debug)]
struct ShipWithWaypointState {
    position: Position,
    waypoint: Position, // relative to ship position
}

impl ShipWithWaypointState {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::MoveInDirection(direction, steps) => {
                self.waypoint = direction.move_position(&self.waypoint, *steps);
            }
            Instruction::Rotate(num_turns) => {
                let [lat, long] = self.waypoint;
                self.waypoint = match num_turns.rem_euclid(4) {
                    0 => [lat, long],
                    1 => [-long, lat],
                    2 => [-lat, -long],
                    3 => [long, -lat],
                    rem => panic!("rem_euclid(4) cannot result in {}", rem),
                }
            }
            Instruction::Forward(steps) => {
                for (ship_coord, waypoint_coord) in self.position.iter_mut().zip(self.waypoint.iter()) {
                    *ship_coord += waypoint_coord * steps;
                }
            }
        }
    }

    fn manhatten_distance(&self) -> usize {
        self.position.iter().map(|coord| coord.abs() as usize).sum()
    }
}

pub fn part_2(input: &[Instruction]) -> usize {
    let mut ship_state = ShipWithWaypointState {
        position: [0, 0],
        waypoint: [1, 10],
    };
    for instruction in input.iter() {
        ship_state.apply_instruction(instruction);
    }
    ship_state.manhatten_distance()
}
