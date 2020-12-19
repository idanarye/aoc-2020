use hashbrown::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
enum Rule {
    Character(char),
    Composition(Vec<Vec<usize>>),
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();
    let rules = it.by_ref().take_while(|&line| line != "").map(|line| {
        let mut parts_it = line.split(": ");
        let rule_number = parts_it.next().unwrap().parse().unwrap();
        let rule_text = parts_it.next().unwrap();
        (rule_number, {
            let mut rule_chars = rule_text.chars();
            if rule_chars.next() == Some('"') {
                Rule::Character(rule_chars.next().unwrap())
            } else {
                Rule::Composition(rule_text.split(" | ").map(|option_text| {
                    option_text.split(" ").map(|rule_num| rule_num.parse().unwrap()).collect()
                }).collect())
            }
        })
    }).collect();
    // numbered_rules.sort_by_key(|(i, _)| *i);
    // for (i, (j, _)) in numbered_rules.iter().enumerate() {
        // assert!(i == *j);
    // }
    // let rules = numbered_rules.into_iter().map(|(_, rule)| rule).collect();

    let messages = it.map(|line| line.to_owned()).collect();

    Input {rules, messages}
}

impl Input {
    fn message_reminder_after_applying_rule<'a>(&self, message: HashSet<&'a str>, rule_number: usize) -> HashSet<&'a str> {
        if message.is_empty() {
            return message;
        }
        match &self.rules[&rule_number] {
            &Rule::Character(c) => message.into_iter().filter_map(|msg| {
                let mut it = msg.chars();
                if it.next() == Some(c) {
                    Some(it.as_str())
                } else {
                    None
                }
            }).collect(),
            Rule::Composition(options) => {
                options.iter().flat_map(|rules_chain| {
                    let mut rest = message.clone();
                    for &rule in rules_chain.iter() {
                        rest = self.message_reminder_after_applying_rule(rest, rule);
                    }
                    rest
                }).collect()
            },
        }
    }

    fn message_matches_rule<'a>(&self, message: &'a str, rule_number: usize) -> bool {
        self.message_reminder_after_applying_rule([message].iter().copied().collect(), rule_number).into_iter().any(|remaining| remaining == "")
    }
}

pub fn part_1(input: &Input) -> usize {
    input.messages.iter().filter(|message| input.message_matches_rule(message, 0)).count()
}

pub fn part_2(input: &Input) -> usize {
    let mut input = input.clone();
    input.rules.insert(8, Rule::Composition(vec![vec![42], vec![42, 8]]));
    input.rules.insert(11, Rule::Composition(vec![vec![42, 31], vec![42, 11, 31]]));
    input.messages.iter().filter(|message| input.message_matches_rule(message, 0)).count()
}
