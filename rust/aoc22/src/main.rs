use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
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

type Deck = VecDeque<u16>;

fn parse_input(input: &str) -> Result<Vec<Deck>> {
    let decks = input
        .split("\n\n")
        .map(|d| {
            d.lines().fold(VecDeque::new(), |mut acc, line| {
                if line.starts_with("Player") {
                    return acc;
                }

                let x = line.parse::<u16>().unwrap();
                acc.push_back(x);
                acc
            })
        })
        .collect::<Vec<VecDeque<_>>>();

    Ok(decks)
}

fn score_deck(deck: &Deck) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (*c as u64) * (i + 1) as u64)
        .sum()
}

fn play_game(decks: &[Deck]) -> u64 {
    let mut p1 = decks[0].clone();
    let mut p2 = decks[1].clone();

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    if p1.is_empty() {
        score_deck(&p2)
    } else {
        score_deck(&p1)
    }
}

fn play_recursive_game(p1: &mut Deck, p2: &mut Deck) -> usize {
    let mut decks_seen = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !decks_seen.insert(hash_decks(p1, p2)) {
            return 1;
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let winner = if (p1.len() >= c1 as usize) && (p2.len() >= c2 as usize) {
            let mut np1 = p1.iter().take(c1 as usize).copied().collect();
            let mut np2 = p2.iter().take(c2 as usize).copied().collect();

            play_recursive_game(&mut np1, &mut np2)
        } else if c1 > c2 {
            1
        } else {
            2
        };

        match winner {
            1 => {
                p1.push_back(c1);
                p1.push_back(c2)
            }
            2 => {
                p2.push_back(c2);
                p2.push_back(c1)
            }
            _ => unreachable!(),
        }
    }

    if p1.is_empty() {
        2
    } else {
        1
    }
}

fn hash_decks(a: &Deck, b: &Deck) -> u64 {
    let mut s = DefaultHasher::new();
    a.hash(&mut s);
    b.hash(&mut s);
    s.finish()
}

fn part1(input: &str) -> Result<()> {
    let decks = parse_input(input)?;
    let score = play_game(&decks);

    println!("part 1 solution: {}", score);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let decks = parse_input(input)?;
    let mut p1 = decks[0].clone();
    let mut p2 = decks[1].clone();

    let winner = play_recursive_game(&mut p1, &mut p2);

    let soln = match winner {
        1 => score_deck(&p1),
        2 => score_deck(&p2),
        _ => unreachable!(),
    };

    println!("part 2 solution: {}", soln);

    Ok(())
}
