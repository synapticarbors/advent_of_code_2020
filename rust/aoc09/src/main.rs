use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A simple HashSet that remembers order of insertion to facilitate
/// a moving window that does not need to be rebuilt each time
#[derive(Debug)]
struct WMHashSet {
    set: HashSet<usize>,
    order: VecDeque<usize>,
}

impl WMHashSet {
    fn new(sz: usize) -> WMHashSet {
        WMHashSet {
            set: HashSet::with_capacity(sz),
            order: VecDeque::with_capacity(sz),
        }
    }

    fn from_iter(d: &[usize]) -> WMHashSet {
        let mut wmh = WMHashSet::new(d.len());
        for i in d {
            wmh.set.insert(*i);
            wmh.order.push_back(*i);
        }

        wmh
    }

    fn contains(&self, v: &usize) -> bool {
        self.set.contains(v)
    }

    fn insert(&mut self, v: &usize) -> bool {
        // Remove oldest value from the set
        if let Some(oldest) = self.order.pop_front() {
            self.set.remove(&oldest);
        }

        self.order.push_back(*v);
        self.set.insert(*v)
    }
}

fn get_min_max_sum(x: &[usize]) -> (usize, usize, usize) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let mut sum = 0;

    for v in x {
        if *v <= min {
            min = *v;
        }

        if *v >= max {
            max = *v;
        }

        sum += *v;
    }

    (min, max, sum)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let start = std::time::Instant::now();
    let target = part1(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    let start = std::time::Instant::now();
    part2(&input, target)?;
    eprintln!("elapsed {:?}", start.elapsed());

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut d = Vec::new();
    for line in input.lines() {
        d.push(line.parse::<usize>()?);
    }
    Ok(d)
}

fn part1(input: &str) -> Result<usize> {
    let data = parse_input(input)?;

    let window_sz = 25;
    let mut wmh = WMHashSet::from_iter(&data[..window_sz]);

    for (i, starget) in data[window_sz..].iter().enumerate() {
        let start = i;
        let end = i + window_sz;

        let mut found = false;

        for v in data[start..end].iter() {
            let compliment = starget - v;
            if *v != compliment && wmh.contains(&compliment) {
                found = true;
                break;
            } else {
                continue;
            }
        }

        if found == false {
            println!("part 1 solution: {}", starget);
            return Ok(*starget);
        }

        wmh.insert(&starget);
    }

    unreachable!();
}

fn part2(input: &str, target: usize) -> Result<()> {
    let data = parse_input(input)?;

    for wsz in 2..input.len() {
        for wdata in data.windows(wsz) {
            let (gmin, gmax, gsum) = get_min_max_sum(wdata);

            if gsum == target {
                println!("part 2 solution: {}", gmin + gmax);
                return Ok(());
            }
        }
    }

    unreachable!();
}
