use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct GameState {
    cups: LinkedList<usize>,
}

pub fn generator(input: &str) -> GameState {
    GameState {
        cups: input.chars().map(|c| c as usize - '0' as usize).collect(),
    }
}

/*
impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cup) in self.cups.iter().enumerate() {
            if 0 < i {
                use std::fmt::Write;
                f.write_char(' ')?;
            }
            if i == self.current_index {
                write!(f, "({})", cup)?;
            } else {
                write!(f, "{}", cup)?;
            }
        }
        Ok(())
    }
}
*/

impl GameState {
    fn run_move(&mut self) {
        let current_value = self.cups.pop_front().unwrap();
        let pick_up: Vec<usize> = (0..3).map(|_| self.cups.pop_front().unwrap()).collect();
        self.cups.push_back(current_value);
        let insert_at_index = self.cups.iter()
            .map(|&cup| {
                (cup as isize - current_value as isize).rem_euclid(10) as usize
            })
            .enumerate()
            .max_by_key(|&(_, key)| key)
            .unwrap().0 + 1;
        let mut cups_after = self.cups.split_off(insert_at_index);
        self.cups.extend(pick_up.into_iter());
        self.cups.append(&mut cups_after);
    }

    fn numbers_after<'a>(&'a self, target: usize) -> impl Iterator<Item = usize> + 'a {
        let one_position = self.cups.iter().position(|&cup| cup == target).unwrap();
        self.cups.iter().skip(one_position + 1).copied()
            .chain(self.cups.iter().take(one_position).copied())
    }
}

pub fn part_1(game: &GameState) -> String {
    let mut game = game.clone();
    for _ in 0..100 {
        game.run_move();
    }
    game.numbers_after(1).map(|cup| format!("{}", cup)).collect::<Vec<_>>().join("")
}

pub fn part_2(game: &GameState) -> usize {
    let mut game = game.clone();

    let max_cup = *game.cups.iter().max().unwrap();
    game.cups.extend(max_cup + 1..=1_000_000);
    for _ in 0..1 {
        game.run_move();
    }

    0
}
