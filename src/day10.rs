use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_1(input: &[usize]) -> usize {
    let mut adapter_jolts: Vec<usize> = input.iter().cloned().collect();
    adapter_jolts.push(0);
    adapter_jolts.sort();
    adapter_jolts.push(adapter_jolts.last().unwrap() + 3);
    let mut differences_count = [0, 0, 0, 0];
    for (low, high) in adapter_jolts.iter().zip(adapter_jolts.iter().skip(1)) {
        differences_count[high - low] += 1;
    }
    differences_count[1] * differences_count[3]
}

pub fn part_2(input: &[usize]) -> usize {
    let mut adapter_jolts: Vec<usize> = input.iter().cloned().collect();
    adapter_jolts.sort();
    adapter_jolts.push(adapter_jolts.last().unwrap() + 3);

    let mut num_ways_to_reach = HashMap::new();
    num_ways_to_reach.insert(0, 1);

    for &aj in adapter_jolts.iter() {
        let min_source = if 3 <= aj {
            aj - 3
        } else {
            0
        };
        let num_ways = (min_source..aj).map(|jolts| num_ways_to_reach.get(&jolts).unwrap_or(&0)).sum();
        num_ways_to_reach.insert(aj, num_ways);
    }
    num_ways_to_reach[adapter_jolts.last().unwrap()]
}
