use std::collections::HashMap;
use std::io::{self, Read};

use peg;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

peg::parser! {
    grammar rules_parser() for str {
        use super::Rule as Rule;
        rule num() -> u16
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule lit() -> Rule
            = "\"" x:$(['a'..='z']) "\"" { Rule::Lit(x.chars().next().unwrap()) }

        rule seq() -> Vec<u16>
            = l:num() ++ " " { l }

        rule lit_rule() -> (u16, Rule)
            = n:num() ":" _ l:lit() { (n, l) }

        rule seq_rule() -> (u16, Rule)
            = n:num() ":" _ l:seq() { (n, Rule::Seq(l)) }

        rule alt_seq_rule() -> (u16, Rule)
            = n:num() ":" _ a:seq() _ "|" _ b:seq() { (n, Rule::Or(a, b)) }

        rule _() = " "?

        pub rule parse_rule_line() -> (u16, Rule)
            = alt_seq_rule() / seq_rule() / lit_rule()

    }
}

#[derive(Debug, PartialEq)]
pub enum Rule {
    Lit(char),
    Seq(Vec<u16>),
    Or(Vec<u16>, Vec<u16>),
}

fn parse_input(input: &str) -> Result<(HashMap<u16, Rule>, &str)> {
    let mut it = input.split("\n\n");
    let raw_rules = it.next().ok_or("Could not split rules")?;
    let raw_messages = it.next().ok_or("Could not split messages")?;

    //let rule_set: HashMap<u16, Rule> = HashMap::new();

    let rule_set = raw_rules
        .lines()
        .map(|line| {
            let x = rules_parser::parse_rule_line(line)?;
            Ok(x)
        })
        .collect::<Result<_>>()?;

    Ok((rule_set, raw_messages))
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
    let (rule_set, messages) = parse_input(input)?;

    println!("{:?}", rule_set);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_lit_test() {
        let line = "16: \"a\"";
        let (n, r) = rules_parser::parse_rule_line(line).unwrap();

        assert_eq!(n, 16);
        assert_eq!(r, Rule::Lit('a'));
    }

    #[test]
    fn part1_seq_test() {
        let line = "16: 1 2 3";
        let (n, r) = rules_parser::parse_rule_line(line).unwrap();

        assert_eq!(n, 16);
        assert_eq!(r, Rule::Seq([1, 2, 3].to_vec()));
    }

    #[test]
    fn part1_alt_seq_test() {
        let line = "1: 1 2 | 3 4";
        let (n, r) = rules_parser::parse_rule_line(line).unwrap();

        assert_eq!(n, 1);
        assert_eq!(r, Rule::Or([1, 2].to_vec(), [3, 4].to_vec()));
    }
}
