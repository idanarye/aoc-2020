#[derive(Debug)]
pub struct Input {
    earliest_depart_time: usize,
    bus_ids: Vec<Option<usize>>,
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();
    let earliest_depart_time = it.next().unwrap().parse().unwrap();
    let bus_ids = it.next().unwrap().split(',').map(|bus_id| {
        match bus_id {
            "x" => None,
            _ => Some(bus_id.parse().unwrap()),
        }
    }).collect();
    Input {
        earliest_depart_time,
        bus_ids,
    }
}

impl Input {
    fn next_arrival_time(&self, bus_id: usize) -> usize {
        let time_since_previous_arrival = self.earliest_depart_time % bus_id;
        let previous_arrival = self.earliest_depart_time - time_since_previous_arrival;
        previous_arrival + bus_id
    }
}

pub fn part_1(input: &Input) -> usize {
    let bus_to_take = input.bus_ids.iter()
        .filter_map(|&x| x)
        .min_by_key(|&bus_id| input.next_arrival_time(bus_id))
        .unwrap();
    bus_to_take * (input.next_arrival_time(bus_to_take) - input.earliest_depart_time)
}

pub fn part_2(input: &Input) -> usize {
    let mut jump_size = 1;
    let mut time = 0;
    for (required_delay, bus_id) in input.bus_ids.iter().enumerate() {
        let bus_id = if let Some(bus_id) = bus_id {
            *bus_id
        } else {
            continue;
        };
        while (time + required_delay) % bus_id != 0 {
            time += jump_size;
        }
        jump_size = num::integer::lcm(jump_size, bus_id);
    }
    time
}
