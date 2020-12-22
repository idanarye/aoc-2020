use std::collections::VecDeque;

use hashbrown::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    decks: [VecDeque<usize>; 2],
}

pub fn generator(input: &str) -> Game {
    let mut it = input.lines();

    assert!(it.next() == Some("Player 1:"));
    let first_player_cards = it.by_ref()
        .take_while(|&line| line != "")
        .map(|line| line.parse().unwrap())
        .collect();

    assert!(it.next() == Some("Player 2:"));
    let second_player_cards = it.by_ref()
        .take_while(|&line| line != "")
        .map(|line| line.parse().unwrap())
        .collect();

    Game { decks: [
        first_player_cards,
        second_player_cards,
    ]}
}

#[derive(Debug)]
enum Winner {
    FirstPlayer,
    SecondPlayer,
}

impl Game {
    fn draw_cards(&mut self) -> [usize; 2] {
        [
            self.decks[0].pop_front().unwrap(),
            self.decks[1].pop_front().unwrap(),
        ]
    }
    fn play_round(&mut self, winning_rule: impl FnOnce(&mut Self, [usize; 2]) -> Winner) {
        let cards = self.draw_cards();
        assert!(cards[0] != cards[1]);
        let winner = winning_rule(self, cards);
        self.push_cards_back(winner, cards);
    }

    fn push_cards_back(&mut self, winner: Winner, drawn_cards: [usize; 2]) {
        match winner {
            Winner::FirstPlayer => {
                self.decks[0].push_back(drawn_cards[0]);
                self.decks[0].push_back(drawn_cards[1]);
            },
            Winner::SecondPlayer => {
                self.decks[1].push_back(drawn_cards[1]);
                self.decks[1].push_back(drawn_cards[0]);
            },
        }
    }

    fn game_winner(&self) -> Option<Winner> {
        if self.decks[0].is_empty() {
            Some(Winner::SecondPlayer)
        } else if self.decks[1].is_empty() {
            Some(Winner::FirstPlayer)
        } else {
            None
        }
    }

    fn calc_score(&self, winner: Winner) -> usize {
        let winner_cards = &self.decks[match winner {
            Winner::FirstPlayer => 0,
            Winner::SecondPlayer => 1,
        }];
        (1..).zip(winner_cards.iter().rev()).map(|(i, &card)| i * card).sum()
    }
}

pub fn part_1(game: &Game) -> usize {
    let mut game = game.clone();
    loop {
        game.play_round(|_this, [first_player_card, second_player_card]| {
            if first_player_card < second_player_card {
                Winner::SecondPlayer
            } else {
                Winner::FirstPlayer
            }
        });
        if let Some(winner) = game.game_winner() {
            return game.calc_score(winner);
        }
    }
}

impl Game {
    fn subgame(&self, cards_to_take: [usize; 2]) -> Game {
        Game {
            decks: [
                self.decks[0].iter().take(cards_to_take[0]).copied().collect(),
                self.decks[1].iter().take(cards_to_take[1]).copied().collect(),
            ],
        }
    }

    fn recursive_play(&mut self) -> Winner {
        let mut past_rounds = HashSet::new();

        loop {
            if !past_rounds.insert(self.clone()) {
                return Winner::FirstPlayer;
            }

            self.play_round(|this, drawn_cards| {
                if drawn_cards.iter().zip(this.decks.iter()).all(|(&drawn, deck)| deck.len() >= drawn) {
                    let mut subgame = this.subgame(drawn_cards);
                    subgame.recursive_play()
                } else {
                    let [first_player_card, second_player_card] = drawn_cards;
                    if first_player_card < second_player_card {
                        Winner::SecondPlayer
                    } else {
                        Winner::FirstPlayer
                    }
                }
            });

            if let Some(winner) = self.game_winner() {
                return winner;
            }
            // break;
        }
    }
}

pub fn part_2(game: &Game) -> usize {
    let mut game = game.clone();
    let winner = game.recursive_play();
    println!("{:?}", winner);
    game.calc_score(winner)
}
