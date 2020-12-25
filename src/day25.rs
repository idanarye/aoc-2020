#[derive(Debug)]
pub struct Input {
    card_public_key: usize,
    door_public_key: usize,
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();
    let card_public_key = it.next().unwrap().parse().unwrap();
    let door_public_key = it.next().unwrap().parse().unwrap();
    Input {card_public_key, door_public_key}
}

struct SubjectNumberTransformer {
    current_value: usize,
    subject_number: usize,
}

impl SubjectNumberTransformer {
    fn new(subject_number: usize) -> Self {
        Self {
            current_value: 1,
            subject_number
        }
    }
}

impl Iterator for SubjectNumberTransformer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_value;
        self.current_value = (self.current_value * self.subject_number) % 20201227;
        Some(result)
    }
}

fn search_loop_sizes1(keys: [usize; 2]) -> [usize; 2] {
    let mut result: [Option<usize>; 2] = [None, None];
    for (loop_size, public_key) in SubjectNumberTransformer::new(7).enumerate() {
        if public_key == keys[0] {
            result[0] = Some(loop_size);
            if result[1].is_some() {
                break;
            }
        } else if public_key == keys[1] {
            result[1] = Some(loop_size);
            if result[0].is_some() {
                break;
            }
        }
    }
    [
        result[0].unwrap(),
        result[1].unwrap(),
    ]
}

pub fn part_1(input: &Input) -> usize {
    let [card_loop_size, _door_loop_size] = search_loop_sizes1([input.card_public_key, input.door_public_key]);
    SubjectNumberTransformer::new(input.door_public_key).nth(card_loop_size).unwrap()
}
