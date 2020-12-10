use std::collections::HashMap;
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

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut adapters = Vec::new();
    adapters.push(0 as usize);

    for line in input.lines() {
        adapters.push(line.parse()?);
    }

    adapters.sort_unstable();
    adapters.push(adapters[adapters.len() - 1] + 3);

    Ok(adapters)
}

fn part1(input: &str) -> Result<()> {
    let adapters = parse_input(input)?;

    let dist = adapters.windows(2).fold(HashMap::new(), |mut acc, v| {
        let diff = v[1] - v[0];
        let e = acc.entry(diff).or_insert(0);
        *e += 1;

        acc
    });

    println!("{:?}", dist);
    let soln = dist.get(&1).unwrap_or(&0) * dist.get(&3).unwrap_or(&0);

    println!("part 1 solution: {:?}", soln);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let adapters = parse_input(input)?;
    let num_adapters = adapters.len();

    let mut path_acc: Vec<usize> = Vec::with_capacity(num_adapters);
    path_acc.resize(num_adapters, 0);

    for (i, v) in adapters.iter().enumerate() {
        match i {
            0 => continue,
            1..=3 => path_acc[i] += 1,
            _ => (),
        }

        let mut ci = i as isize - 1;

        while (ci > 0) && (v - adapters[ci as usize] <= 3) {
            path_acc[i] += path_acc[ci as usize];
            ci -= 1;
        }
    }

    let num_arangements = path_acc[num_adapters - 1];

    println!("part 2 solution: {:?}", num_arangements);
    Ok(())
}
