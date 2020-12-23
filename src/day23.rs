#[derive(Debug, Clone)]
pub struct GameState {
    nexts: Vec<usize>,
}

pub fn generator(input: &str) -> GameState {
    let mut nexts = vec![0; 10];
    let mut current = 0;
    for c in input.chars() {
        let label = c as usize - '0' as usize;
        nexts[current] = label;
        current = label;
    }
    nexts[current] = nexts[0];
    GameState { nexts }
}

impl GameState {
    fn iter_from(&self, start_from: usize) -> impl Iterator<Item = usize> + '_ {
        let first = self.nexts[start_from];
        std::iter::successors(Some(first), move |&cup| {
            let next = self.nexts[cup];
            if next == first {
                None
            } else {
                Some(next)
            }
        })
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.iter_from(0);
        write!(f, "({})", it.next().unwrap())?;
        for cup in it {
            write!(f, " {}", cup)?;
        }
        Ok(())
    }
}

impl GameState {
    fn run_move(&mut self) {
        let mut pick_up: [usize; 3] = [0, 0, 0];
        for (c, p) in self.iter_from(0).skip(1).zip(pick_up.iter_mut()) {
            *p = c;
        }
        let mut put_after = self.nexts[0] - 1;
        loop {
            if put_after < 1 {
                put_after = self.nexts.len() - 1;
            }
            if pick_up.iter().any(|&c| c == put_after) {
                put_after -= 1;
            } else {
                break;
            }
        }
        let first_number = self.nexts[0];
        self.nexts[first_number] = self.nexts[pick_up[2]];
        self.nexts[pick_up[2]] = self.nexts[put_after];
        self.nexts[put_after] = pick_up[0];
        self.nexts[0] = self.nexts[self.nexts[0]];
    }
}

pub fn part_1(game: &GameState) -> String {
    let mut game = game.clone();
    for _ in 0..100 {
        game.run_move();
    }
    game.iter_from(1).take_while(|&cup| cup != 1).map(|cup| format!("{}", cup)).collect::<Vec<_>>().join("")
}

pub fn part_2(game: &GameState) -> usize {
    let mut game = game.clone();
    let last_cup = game.iter_from(0).last().unwrap();
    game.nexts[last_cup] = game.nexts.len();
    let goal = 1_000_000;
    game.nexts.extend((game.nexts.len() + 1)..=goal);
    game.nexts.push(game.nexts[0]);

    for _ in 0..10_000_000 {
        game.run_move();
    }

    if let [a, b] = game.iter_from(1).take(2).collect::<Vec<_>>().as_slice() {
        a * b
    } else {
        panic!("Expected to take 2");
    }
}
