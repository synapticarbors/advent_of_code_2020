use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::FromIterator;

use peg;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

peg::parser! {
    grammar food_parser() for str {
        rule word() -> &'input str
            = w:$(['a'..='z']+) { w }

        rule ingredients() -> Vec<&'input str>
            = w:word() ++ " " { w }

        rule allergens() -> Vec<&'input str>
            = "(contains" _ w:word() ++ ", " ")" { w }

        rule _() = " "?

        pub rule parse_line() -> (HashSet<&'input str>, HashSet<&'input str>)
            = i:ingredients() _ a:allergens() {
                (HashSet::from_iter(i.iter().map(|g| *g)),
                 HashSet::from_iter(a.iter().map(|g| *g))) }
    }

}

#[derive(Debug, Default)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
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

fn parse_input(input: &str) -> Result<Vec<Food>> {
    let mut foods = vec![];
    for line in input.lines() {
        let (i, a) = food_parser::parse_line(line)?;
        foods.push(Food {
            ingredients: i,
            allergens: a,
        });
    }

    Ok(foods)
}

fn identify_allergens<'a>(foods: &[Food<'a>]) -> Result<HashMap<&'a str, &'a str>> {
    let mut allergen_candidates: HashMap<&str, HashSet<&str>> = HashMap::new();

    for f in foods.iter() {
        for a in f.allergens.iter() {
            let e = allergen_candidates.entry(*a).or_insert(HashSet::new());
            if e.len() == 0 {
                (*e).extend(&f.ingredients);
            }
            *e = e.intersection(&f.ingredients).map(|&x| x).collect();
        }
    }

    let mut a2i = HashMap::new();

    while let Some((&a, _)) = allergen_candidates.iter().find(|(_, s)| s.len() == 1) {
        if let Some(&i) = allergen_candidates[a].iter().next() {
            a2i.insert(a, i);

            for h in allergen_candidates.values_mut() {
                h.remove(&i);
            }
        }
    }

    Ok(a2i)
}

fn part1(input: &str) -> Result<()> {
    let foods = parse_input(input)?;
    let a2i = identify_allergens(&foods)?;
    let allergen_ingredients = a2i.values().collect::<HashSet<_>>();
    let mut cnt = 0;

    for f in foods.iter() {
        cnt += f
            .ingredients
            .iter()
            .filter(|&i| !allergen_ingredients.contains(&i))
            .count();
    }

    println!("part 1 solution: {}", cnt);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let foods = parse_input(input)?;
    let a2i = identify_allergens(&foods)?;

    let mut sorted_allergens = a2i.keys().collect::<Vec<_>>();
    sorted_allergens.sort();

    let mut x = vec![];
    for a in sorted_allergens.iter() {
        if let Some(i) = a2i.get(*a) {
            x.push(*i);
        }
    }

    let soln = x.join(",");

    println!("part 2 solution: {}", soln);

    Ok(())
}
