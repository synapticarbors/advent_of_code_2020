use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Range = std::ops::Range<u16>;

#[derive(Debug)]
enum InputSection {
    Rules,
    MyTicket,
    NearbyTicket,
}

impl InputSection {
    fn next(&self) -> InputSection {
        match self {
            InputSection::Rules => InputSection::MyTicket,
            InputSection::MyTicket => InputSection::NearbyTicket,
            InputSection::NearbyTicket => InputSection::Rules,
        }
    }
}

fn parse_range(x: &Captures, name: &str) -> Result<u16> {
    let y = x
        .name(name)
        .ok_or(format!("no {}", name))?
        .as_str()
        .parse::<u16>()?;
    Ok(y)
}

#[derive(Debug, Default)]
struct Notes {
    rules: HashMap<String, Vec<Range>>,
    my_ticket: Vec<u16>,
    nearby_tickets: Vec<Vec<u16>>,
}

impl Notes {
    fn parse_rule(line: &str) -> Result<(String, Vec<Range>)> {
        lazy_static! {
            static ref RE_RULES: Regex = Regex::new(
                r"(?P<rule_name>^.*): (?P<n1>\d+)-(?P<n2>\d+) or (?P<n3>\d+)-(?P<n4>\d+)"
            )
            .unwrap();
        }

        if let Some(x) = RE_RULES.captures(line) {
            let mut r: Vec<Range> = Vec::with_capacity(2);
            r.push(Range {
                start: parse_range(&x, "n1")?,
                end: parse_range(&x, "n2")? + 1,
            });

            r.push(Range {
                start: parse_range(&x, "n3")?,
                end: parse_range(&x, "n4")? + 1,
            });

            let rule_name = x
                .name("rule_name")
                .ok_or("no rule name")?
                .as_str()
                .to_string();

            return Ok((rule_name, r));
        }

        Err("could not parse rule line".to_string().into())
    }

    fn parse_ticket(line: &str) -> Result<Vec<u16>> {
        let mut t = vec![];
        for x in line.split(",") {
            t.push(x.parse::<u16>()?);
        }

        Ok(t)
    }

    fn from_input(input: &str, skip_invalid: bool) -> Result<Notes> {
        let mut notes = Notes::default();

        let mut section = InputSection::Rules;

        for line in input.lines() {
            if line.len() == 0 {
                section = section.next();
            };

            match section {
                InputSection::Rules => {
                    let (rule_name, ranges) = Notes::parse_rule(line)?;
                    notes.rules.insert(rule_name, ranges);
                }
                InputSection::MyTicket => {
                    if let Ok(x) = Notes::parse_ticket(line) {
                        notes.my_ticket = x;
                    }
                }
                InputSection::NearbyTicket => {
                    if let Ok(x) = Notes::parse_ticket(line) {
                        if skip_invalid & !notes.is_ticket_valid(&x) {
                            continue;
                        }
                        notes.nearby_tickets.push(x);
                    }
                }
            }
        }

        Ok(notes)
    }

    fn is_num_valid(&self, x: &u16) -> bool {
        for rule in self.rules.values().flatten() {
            if rule.contains(&x) {
                return true;
            }
        }
        false
    }

    fn is_ticket_valid(&self, ticket: &[u16]) -> bool {
        for n in ticket {
            if !self.is_num_valid(n) {
                return false;
            }
        }
        true
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let start = std::time::Instant::now();
    part1(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    let start = std::time::Instant::now();
    part2(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let notes = Notes::from_input(input, false)?;

    let error_rate: u32 = notes
        .nearby_tickets
        .iter()
        .flatten()
        .map(|x| match notes.is_num_valid(x) {
            true => 0,
            false => *x as u32,
        })
        .sum();

    println!("part 1 solution: {}", error_rate);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let notes = Notes::from_input(input, true)?;

    let num_fields = notes.my_ticket.len();
    let mut field_candidates: Vec<HashSet<String>> = Vec::new();

    for _ in 0..num_fields {
        field_candidates.push(HashSet::new());
    }

    for i in 0..num_fields {
        let field_values: Vec<u16> = notes.nearby_tickets.iter().map(|x| x[i]).collect();

        for (rule_name, rules) in &notes.rules {
            let mut rule_valid = true;

            for fv in field_values.iter() {
                if !rules.iter().any(|v| v.contains(&fv)) {
                    rule_valid = false;
                    break;
                }
            }

            if rule_valid {
                field_candidates[i].insert(rule_name.to_owned());
            }
        }
    }

    let mut field2col = HashMap::new();

    // Scan through candidates for columns with only one candidate
    // and then remove from all others
    while field2col.len() < num_fields {
        for field in field2col.keys() {
            for candidate in field_candidates.iter_mut() {
                candidate.remove(field);
            }
        }

        for (i, candidates) in field_candidates.iter_mut().enumerate() {
            if candidates.is_empty() {
                continue;
            } else if candidates.len() == 1 {
                for n in candidates.drain() {
                    field2col.insert(n, i);
                }
            }
        }
    }

    let soln: u64 = field2col
        .iter()
        .map(|(name, &i)| match name.starts_with("departure") {
            true => notes.my_ticket[i] as u64,
            false => 1,
        })
        .product();

    println!("part 2 solution: {}", soln);

    Ok(())
}
