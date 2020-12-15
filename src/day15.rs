use std::num::NonZeroUsize;

pub fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Game {
    last_round: usize,
    next_number: usize,
    last_seen_times: Vec<Option<NonZeroUsize>>
}

impl Game {
    fn new() -> Self {
        Self {
            last_round: 0,
            next_number: 0,
            last_seen_times: Vec::new(),
        }
    }

    fn feed_number(&mut self, number: usize) {
        self.last_round += 1;
        if self.last_seen_times.len() <= number {
            self.last_seen_times.resize(number + 1, None);
        }
        self.next_number = if let Some(last_seen_time) = self.last_seen_times[number] {
            self.last_round - last_seen_time.get()
        } else {
            0
        };
        self.last_seen_times[number] = NonZeroUsize::new(self.last_round);
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
    game.nth(30000000 - input.len() - 1).unwrap()
}
