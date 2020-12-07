use std::collections::HashMap;

use regex::Regex;

type RowData = HashMap<String, String>;

pub fn generator(input: &str) -> Vec<RowData> {
    let pattern = Regex::new(r"(\w+):(\S+)").unwrap();

    let mut result = Vec::new();

    let mut it = input.lines();
    let mut current = HashMap::new();

    loop {
        match it.next() {
            Some("") => {
                if !current.is_empty() {
                    result.push(current);
                    current = HashMap::new();
                }
            }
            Some(line) => {
                for captures in pattern.captures_iter(line) {
                    current.insert(captures[1].to_owned(), captures[2].to_owned());
                }
            }
            None => {
                result.push(current);
                return result;
            }
        }
    }
}

pub fn part_1(input: &[RowData]) -> usize {
    let mandatory_fields  = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ];

    input.iter().filter(|entry| {
        for mandatory_field in mandatory_fields.iter() {
            if !entry.contains_key(mandatory_field as &str) {
                return false;
            }
        }
        true
    }).count()
}

pub fn part_2(input: &[RowData]) -> usize {
    let year_pattern = Regex::new(r"^\d{4}$").unwrap();
    let verify_year = |value, range: std::ops::RangeInclusive<usize>| {
        if !year_pattern.is_match(value) {
            return None;
        }
        let year: usize = value.parse().ok()?;
        if !range.contains(&year) {
            return None;
        }
        Some(())
    };

    let height_pattern = Regex::new(r"^(\d+)(.*)$").unwrap();
    let hair_color_pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_color_pattern = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_pattern = Regex::new(r"^\d{9}$").unwrap();

    input.iter().filter_map(|entry| {
        verify_year(entry.get("byr")?, 1920..=2002)?;
        verify_year(entry.get("iyr")?, 2010..=2020)?;
        verify_year(entry.get("eyr")?, 2020..=2030)?;

        let hgt = height_pattern.captures(entry.get("hgt")?)?;
        let hgt_range = match hgt.get(2)?.as_str() {
            "cm" => 150..=193,
            "in" => 59..=76,
            _ => return None,
        };
        if !hgt_range.contains(&hgt.get(1)?.as_str().parse::<usize>().unwrap()) {
            return None;
        }

        let hcl = entry.get("hcl")?;
        if !hair_color_pattern.is_match(hcl) {
            return None;
        }

        let ecl = entry.get("ecl")?;
        if !eye_color_pattern.is_match(ecl) {
            return None;
        }

        let pid = entry.get("pid")?;
        if !pid_pattern.is_match(pid) {
            return None;
        }

        Some(())
    }).count()
}
