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

fn part1(input: &str) -> Result<()> {
    let group_answers = input
        .lines()
        .fold(vec![HashSet::with_capacity(26)], |mut acc, v| {
            if v.is_empty() {
                acc.push(HashSet::with_capacity(26));
                acc
            } else {
                if let Some(hs) = acc.last_mut() {
                    v.chars().for_each(|g| {
                        hs.insert(g);
                    });
                }
                acc
            }
        });
    let total: usize = group_answers.iter().map(|g| g.len()).sum();
    println!("part 1 solution: {}", total);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut gid = 0;
    let mut hs_cmp = HashSet::with_capacity(26);

    let group_common_answers =
        input
            .lines()
            .fold(vec![HashSet::with_capacity(26)], |mut acc, v| {
                if v.is_empty() {
                    gid = 0;
                    acc.push(HashSet::with_capacity(26));
                    acc
                } else {
                    if gid == 0 {
                        if let Some(hs) = acc.last_mut() {
                            v.chars().for_each(|g| {
                                hs.insert(g);
                            });
                        }
                    } else {
                        hs_cmp.clear();
                        v.chars().for_each(|g| {
                            hs_cmp.insert(g);
                        });

                        if let Some(hs) = acc.last_mut() {
                            let x: HashSet<char> = hs.intersection(&hs_cmp).copied().collect();
                            hs.clear();
                            hs.extend(&x);
                        }
                    }

                    gid += 1;
                    acc
                }
            });

    let total: usize = group_common_answers.iter().map(|g| g.len()).sum();
    println!("part 2 solution: {}", total);

    Ok(())
}
