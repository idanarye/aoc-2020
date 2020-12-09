use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_1(input: &[isize]) -> isize {
    for (i, &target_number) in input.iter().enumerate().skip(25) {
        let number_pool: HashSet<isize> = input[i - 25..i].iter().cloned().collect();
        if !number_pool.iter().any(|&candidate| {
            let complement = target_number - candidate;
            complement != candidate && number_pool.contains(&complement)
        }) {
            return target_number;
        }
    }
    panic!("No violating number");
}

pub fn part_2(input: &[isize]) -> isize {
    let invalid_number = part_1(input);
    for start_position in 0..input.len() {
        let first_number = input[start_position];
        let mut range_sum = first_number;
        let mut smallest = first_number;
        let mut largest = first_number;
        for &other in input[start_position + 1..].iter() {
            if other < smallest {
                smallest = other;
            }
            if largest < other {
                largest = other;
            }
            range_sum += other;
            if invalid_number < range_sum {
                break;
            }
            if range_sum == invalid_number {
                return smallest + largest;
            }
        }
    }
    panic!("No matching range");
}
