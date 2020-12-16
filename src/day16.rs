use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Field {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
    mask: usize,
}

#[derive(Debug)]
pub struct Ticket(Vec<usize>);

#[derive(Debug)]
pub struct Input {
    fields: Vec<Field>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl From<&str> for Ticket {
    fn from(line: &str) -> Self {
        Self(line.split(',').map(|num| num.parse().unwrap()).collect())
    }
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();

    let fields = it.by_ref().take_while(|&line| line != "").enumerate().map(|(idx, line)| {
        let mut parts = line.split(": ");

        let name = parts.next().unwrap().to_owned();

        let ranges = parts.next().unwrap().split(" or ").map(|range| {
            let mut parts = range.split('-');
            let from = parts.next().unwrap().parse().unwrap();
            let to = parts.next().unwrap().parse().unwrap();
            from..=to
        }).collect();

        assert!(parts.next() == None);

        let mask = 1 << idx;

        Field {name, ranges, mask}
    }).collect();

    assert!(it.next() == Some("your ticket:"));
    let my_ticket = it.next().unwrap().into();

    assert!(it.next() == Some(""));

    assert!(it.next() == Some("nearby tickets:"));
    let nearby_tickets = it.map(|line| line.into()).collect();

    Input {fields, my_ticket, nearby_tickets}
}

impl Field {
    fn is_in_range(&self, number: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&number))
    }
}

impl Ticket {
    fn is_valid(&self, input: &Input) -> bool {
        self.0.iter().all(|&number| input.fields.iter().any(|field| field.is_in_range(number)))
    }
}

pub fn part_1(input: &Input) -> usize {
    input.nearby_tickets.iter()
        .flat_map(|ticket| ticket.0.iter())
        .filter(|&number| !input.fields.iter().any(|field| field.is_in_range(*number)))
        .sum()
}

pub fn part_2(input: &Input) -> usize {
    let mut slots_masks = vec![(1 << input.fields.len()) - 1; input.fields.len()];
    let valid_tickets = input.nearby_tickets.iter().filter(|t| t.is_valid(input)).collect::<Vec<_>>();
    for ticket in valid_tickets {
        for (value, slot_mask) in ticket.0.iter().zip(slots_masks.iter_mut()) {
            for field in input.fields.iter() {
                if !field.is_in_range(*value) {
                    *slot_mask &= !field.mask;
                }
            }
        }
    }

    let mut verified_slots = vec![None; slots_masks.len()];
    loop {
        if let Some(next_verified) = slots_masks.iter().zip(verified_slots.iter())
            .position(|(mask, &verified)| verified == None && mask.count_ones() == 1)
        {
            let field_index = slots_masks[next_verified].trailing_zeros() as usize;
            let field = &input.fields[field_index];
            verified_slots[next_verified] = Some(field.name.as_str());
            assert!(slots_masks[next_verified] == field.mask);
            for (i, slot_mask) in slots_masks.iter_mut().enumerate() {
                if i != next_verified {
                    *slot_mask &= !field.mask;
                }
            }
        } else {
            break;
        }
    }

    let fields_by_slot = verified_slots.into_iter().map(Option::unwrap);
    fields_by_slot.zip(input.my_ticket.0.iter())
        .filter_map(|(field_name, value)| {
            if field_name.starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .product()
}
