use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

struct SolveHelper<'a> {
    orig_order: &'a [isize],
    hashed: HashSet<isize>,
}

impl<'a> SolveHelper<'a> {
    fn new(input: &'a [isize]) -> Self {
        Self {
            orig_order: input,
            hashed: input.iter().cloned().collect(),
        }
    }

    fn find_match_for(&self, target: isize, allowed_numbers: isize) -> Option<isize> {
        match allowed_numbers {
            0 => None,
            1 => if self.hashed.contains(&target) {
                Some(target)
            } else {
                None
            },
            _ => {
                for &number in self.orig_order.iter() {
                    if let Some(result) = self.find_match_for(target - number, allowed_numbers - 1) {
                        return Some(result * number)
                    }
                }
                None
            }
        }
    }
}

pub fn part_1(input: &[isize]) -> isize {
    SolveHelper::new(input).find_match_for(2020, 2).unwrap()
}

pub fn part_2(input: &[isize]) -> isize {
    SolveHelper::new(input).find_match_for(2020, 3).unwrap()
}
