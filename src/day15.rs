use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Game {
    next_number: usize,
    ages: HashMap<usize, usize>,
}

impl Game {
    fn new() -> Self {
        Self {
            next_number: 0,
            ages: HashMap::default(),
        }
    }

    fn feed_number(&mut self, number: usize) {
        for age in self.ages.values_mut() {
            *age += 1;
        }
        self.next_number = match self.ages.entry(number) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(0);
                0
            },
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let result = *entry.get();
                *(entry.get_mut()) = 0;
                result
            },
        };
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = self.next_number;
        self.feed_number(self.next_number);
        Some(result)
    }
}

pub fn part_1(input: &[usize]) -> usize {
    let mut game = Game::new();
    for number in input.iter() {
        game.feed_number(*number);
    }
    game.nth(2020 - input.len() - 1).unwrap()
}

pub fn part_2(input: &[usize]) -> usize {
    let mut game = Game::new();
    for number in input.iter() {
        game.feed_number(*number);
    }
    // game.nth(30000000 - input.len() - 1).unwrap()
    0
}
