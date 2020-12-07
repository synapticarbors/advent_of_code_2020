use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

#[derive(Debug)]
struct Bag<'a> {
    pub color: &'a str,
    pub rules: Vec<BagRule<'a>>,
}

#[derive(Debug)]
struct BagRule<'a> {
    pub color: &'a str,
    pub count: u16,
}

impl<'a> Bag<'a> {
    fn from_line(line: &'a str) -> Result<Bag> {
        let mut it = line.split(" bags contain ");
        let color = it.next().ok_or("no color")?;

        let rules_sec = it.next().ok_or("no rules section")?;

        if rules_sec.starts_with("no other bags") {
            return Ok(Bag {
                color: color,
                rules: Vec::new(),
            });
        }

        let rules = rules_sec[..rules_sec.len() - 1]
            .split(", ")
            .map(|g| {
                let mut x = g.splitn(2, " ");
                let num = x.next().ok_or("no num on rule")?.parse::<u16>()?;

                let c = x.next().ok_or("")?.split(" bag").next().ok_or("")?;

                Ok(BagRule {
                    color: c,
                    count: num,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Bag { color, rules })
    }
}

fn parse_input(input: &str) -> Result<Vec<Bag>> {
    input
        .lines()
        .map(|line| Bag::from_line(line))
        .collect::<Result<Vec<_>>>()
}

fn search_inner_bags<'a>(bags: &'a [Bag], target: &'a str) -> HashSet<&'a str> {
    let contains_target = bags
        .iter()
        .filter(|b| b.rules.iter().any(|x| x.color == target))
        .map(|b| b.color)
        .collect::<HashSet<_>>();

    contains_target
        .iter()
        .fold(contains_target.clone(), |mut acc, t| {
            acc.extend(search_inner_bags(bags, t));
            acc
        })
}

fn count_inner_bags<'a>(bags: &'a [Bag], target: &'a str) -> usize {
    if let Some(bag) = bags.iter().find(|b| b.color == target) {
        bag.rules
            .iter()
            .map(|b| b.count as usize * count_inner_bags(bags, b.color))
            .sum::<usize>()
            + 1
    } else {
        1
    }
}

fn part1(input: &str) -> Result<()> {
    let bags = parse_input(input)?;
    let num_bags = search_inner_bags(&bags, "shiny gold").len();
    println!("part 1 solution: {}", num_bags);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let bags = parse_input(input)?;
    let num_bags = count_inner_bags(&bags, "shiny gold") - 1;
    println!("part 2 solution: {}", num_bags);
    Ok(())
}
