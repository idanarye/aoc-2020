#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(usize),
    Plus,
    Times,
    Open,
    Close,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(num) => write!(f, "{}", num),
            Token::Plus => write!(f, "+"),
            Token::Times => write!(f, "*"),
            Token::Open => write!(f, "("),
            Token::Close => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct Equation(Vec<Token>);

impl core::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.0.iter() {
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

pub fn generator(equations: &str) -> Vec<Equation> {
    let pattern = regex::Regex::new(r"\d+|[+*()]").unwrap();
    equations.lines().map(|line| {
        Equation(
        pattern.find_iter(line).map(|m| match m.as_str() {
            "+" => Token::Plus,
            "*" => Token::Times,
            "(" => Token::Open,
            ")" => Token::Close,
            num => Token::Number(num.parse().unwrap()),
        }).collect())
    }).collect()
}

fn solve_1(tokens: &mut dyn Iterator<Item = Token>) -> usize {
    #[derive(Debug)]
    enum State {
        Number(usize),
        Add(usize),
        Multiply(usize),
    }
    impl State {
        fn apply(&mut self, right: usize) {
            *self = match self {
                State::Number(_) => panic!("Illegal application while {:?}", self),
                State::Add(left) => State::Number(*left + right),
                State::Multiply(left) => State::Number(*left * right),
            };
        }
    }
    let mut state = State::Add(0);
    while let Some(token) = tokens.next() {
        match token {
            Token::Number(num) => {
                state.apply(num);
            },
            Token::Plus => {
                match state {
                    State::Number(num) => {
                        state = State::Add(num);
                    }
                    _ => panic!("Illegal {:?} while {:?}", token, state),
                }
            }
            Token::Times => {
                match state {
                    State::Number(num) => {
                        state = State::Multiply(num);
                    }
                    _ => panic!("Illegal {:?} while {:?}", token, state),
                }
            }
            Token::Open => {
                state.apply(solve_1(tokens));
            }
            Token::Close => {
                break;
            }
        }
    }
    if let State::Number(result) = state {
        result
    } else {
        panic!("Unfinished resolution at {:?}", state);
    }
}

impl Equation {
    fn solve_1(&self) -> usize {
        solve_1(&mut self.0.iter().copied())
    }
}

pub fn part_1(equations: &[Equation]) -> usize {
    equations.iter().map(|equation| equation.solve_1()).sum()
}

fn solve_2(tokens: &mut dyn Iterator<Item = Token>) -> usize {
    #[derive(Debug)]
    enum Action {
        AndMultiply(usize),
        AndAdd(usize),
        Terminal(usize),
    }
    let mut actions = Vec::new();
    while let Some(token) = tokens.next() {
        let number = match token {
            Token::Number(num) => num,
            Token::Open => solve_2(tokens),
            ilg => panic!("{:?} is illegal here", ilg),
        };
        match tokens.next() {
            Some(Token::Times) => {
                actions.push(Action::AndMultiply(number));
            },
            Some(Token::Plus) => {
                actions.push(Action::AndAdd(number));
            },
            None | Some(Token::Close) => {
                actions.push(Action::Terminal(number));
                break;
            },
            ilg => panic!("{:?} is illegal here", ilg),
        }
    }
    actions.reverse(); // From this point on we want to treat it like a stack

    impl Action {
        fn modify_number(&mut self, dlg: impl FnOnce(usize) -> usize) {
            match self {
                Action::AndMultiply(number) | Action::AndAdd(number) | Action::Terminal(number) => {
                    *number = dlg(*number);
                },
            }
        }
    }

    fn collapse_additions(actions: &mut Vec<Action>) {
        while let Some(Action::AndAdd(left)) = actions.last() {
            let left = *left; // release the borrow so we can pop
            actions.pop().unwrap();
            actions.last_mut().unwrap().modify_number(|right| left + right);
        }
    }

    fn collapse_multiplications(actions: &mut Vec<Action>) {
        loop {
            match actions.last().unwrap() {
                Action::AndAdd(_) => {
                    collapse_additions(actions);
                },
                Action::AndMultiply(left) => {
                    let left = *left; // release the borrow so we can pop
                    actions.pop().unwrap();
                    collapse_additions(actions);
                    actions.last_mut().unwrap().modify_number(|right| left * right);
                },
                Action::Terminal(_) => {
                    return;
                },
            }
        }
    }

    collapse_multiplications(&mut actions);
    if actions.len() == 1 {
        if let Action::Terminal(result) = actions[0] {
            return result;
        }
    }
    panic!("Expected single terminal value, not {:?}", actions);
}

impl Equation {
    fn solve_2(&self) -> usize {
        solve_2(&mut self.0.iter().copied())
    }
}

pub fn part_2(equations: &[Equation]) -> usize {
    equations.iter().map(|equation| equation.solve_2()).sum()
}
