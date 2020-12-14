use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Mask {
    bitwise_and: usize,
    bitwise_or: usize,
}

#[derive(Debug)]
pub enum Instruction {
    Mask(Mask),
    Memory {
        address: usize,
        value: usize,
    },
}

pub fn generator(input: &str) -> Vec<Instruction> {
    let pattern = regex::Regex::new(r"^(?:mask|mem\[(\d+)\]) = (.*)$").unwrap();
    input.lines().map(|line| {
        let m = pattern.captures(line).unwrap();
        match m.get(1) {
            None => {
                let mut bitwise_and = 0;
                let mut bitwise_or = 0;
                for c in m[2].chars() {
                    bitwise_and <<= 1;
                    bitwise_or <<= 1;
                    match c {
                        '0' => {
                        },
                        '1' => {
                            bitwise_and |= 1;
                            bitwise_or |= 1;
                        },
                        'X' => {
                            bitwise_and |= 1;
                        },
                        _ => panic!("Illegal mask character {:?}", c),
                    }
                }
                Instruction::Mask(Mask {
                    bitwise_and,
                    bitwise_or,
                })
            },
            Some(address) => {
                Instruction::Memory {
                    address: address.as_str().parse().unwrap(),
                    value: m[2].parse().unwrap(),
                }
            },
        }
    }).collect()
}

struct ComputerState {
    mask: Mask,
    memory: HashMap<usize, usize>,
}

impl Mask {
    fn apply(&self, value: usize) -> usize {
        (value & self.bitwise_and) | self.bitwise_or
    }

    fn all_floating_submasks(&self) -> impl Iterator<Item = usize> {
        let places_of_difference = self.bitwise_and ^ self.bitwise_or;
        std::iter::successors(Some((places_of_difference, 1)), |&(places_of_difference, curr_mask)| {
            if places_of_difference == 0 {
                return None;
            }
            let new_places_of_difference = places_of_difference >> 1;
            let new_curr_mask = curr_mask << 1;
            Some((new_places_of_difference, new_curr_mask))
        }).filter_map(|(places_of_difference, curr_mask)| {
            if places_of_difference % 2 == 1 {
                Some(curr_mask)
            } else {
                None
            }
        })
    }

    fn all_floating_possibilities(&self, address: usize) -> impl Iterator<Item = usize> {
        let address = address | self.bitwise_or;
        let floating_submasks: Vec<_> = self.all_floating_submasks().collect();
        (0..(1 << floating_submasks.len())).map(move |mut choice| {
            let mut address = address;
            for submask in floating_submasks.iter() {
                address |= submask;
                if choice % 2 == 0 {
                    address -= submask;
                }
                choice >>= 1;
            }
            address
        })
    }
}

pub fn part_1(input: &[Instruction]) -> usize {
    let mut state = ComputerState {
        mask: Mask {
            bitwise_and: (1 << 36) - 1,
            bitwise_or: 0,
        },
        memory: HashMap::default(),
    };
    for instruction in input.iter() {
        match instruction {
            Instruction::Mask(mask) => {
                state.mask = mask.clone();
            },
            Instruction::Memory { address, value } => {
                state.memory.insert(*address, state.mask.apply(*value));
            },
        }
    }
    state.memory.values().sum()
}

pub fn part_2(input: &[Instruction]) -> usize {
    let mut state = ComputerState {
        mask: Mask {
            bitwise_and: (1 << 36) - 1,
            bitwise_or: 0,
        },
        memory: HashMap::default(),
    };
    for instruction in input.iter() {
        match instruction {
            Instruction::Mask(mask) => {
                state.mask = mask.clone();
            },
            Instruction::Memory { address, value } => {
                for floating_address in state.mask.all_floating_possibilities(*address) {
                    state.memory.insert(floating_address, *value);
                }
            },
        }
    }
    state.memory.values().sum()
}
