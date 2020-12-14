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

#[derive(Debug)]
struct ProgramInstruction {
    loc: usize,
    value: u64,
    set_mask: u64,
    clear_mask: u64,
}

#[derive(Debug)]
struct ProgramInstructionV2 {
    loc: usize,
    value: u64,
    set_mask: u64,
    floating: Vec<u64>,
}

fn parse_input(input: &str) -> Result<Vec<ProgramInstruction>> {
    let mut pi = vec![];

    let mut curr_set_mask: u64 = 0;
    let mut curr_clear_mask: u64 = std::u64::MAX;

    for line in input.lines() {
        if line.starts_with("mask") {
            curr_set_mask = line
                .replace("mask = ", "")
                .trim()
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == '1')
                .fold(0, |acc, (i, _)| acc | (1 << i));

            curr_clear_mask = line
                .replace("mask = ", "")
                .trim()
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == '0')
                .fold(std::u64::MAX, |acc, (i, _)| acc & !(1 << i));
        } else if line.starts_with("mem") {
            let start = line.find("[").ok_or("could not find opening [")?;
            let stop = line.find("]").ok_or("could not find closing ]")?;
            let loc = line[start + 1 as usize..stop].parse::<usize>()?;
            let value = line
                .split("=")
                .skip(1)
                .next()
                .ok_or("")?
                .trim()
                .parse::<u64>()?;

            pi.push(ProgramInstruction {
                loc: loc,
                value: value,
                set_mask: curr_set_mask,
                clear_mask: curr_clear_mask,
            });
        }
    }

    Ok(pi)
}

fn parse_input_p2(input: &str) -> Result<Vec<ProgramInstructionV2>> {
    let mut pi = vec![];

    let mut curr_set_mask: u64 = 0;
    let mut curr_floating: Vec<u64> = Vec::with_capacity(36);

    for line in input.lines() {
        if line.starts_with("mask") {
            let mask_str = line[7..].trim();

            curr_set_mask = mask_str
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == '1')
                .fold(0, |acc, (i, _)| acc | (1 << i));

            let floating_mask: u64 = mask_str
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .fold(0, |acc, (i, _)| acc | (1 << i));

            curr_floating.clear();
            for i in 0..36 {
                if (floating_mask & (1 << i)) != 0 {
                    curr_floating.push(i);
                }
            }
        } else if line.starts_with("mem") {
            let start = line.find("[").ok_or("could not find opening [")?;
            let stop = line.find("]").ok_or("could not find closing ]")?;
            let loc = line[start + 1 as usize..stop].parse::<usize>()?;
            let value = line
                .split("=")
                .skip(1)
                .next()
                .ok_or("")?
                .trim()
                .parse::<u64>()?;

            pi.push(ProgramInstructionV2 {
                loc: loc,
                value: value,
                set_mask: curr_set_mask,
                floating: curr_floating.clone(),
            });
        }
    }

    Ok(pi)
}

fn part1(input: &str) -> Result<()> {
    let instructions = parse_input(input)?;
    let mut dockmem: HashMap<usize, u64> = HashMap::new();

    for instr in instructions {
        let mut value = instr.value;
        value |= instr.set_mask;
        value &= instr.clear_mask;

        let e = dockmem.entry(instr.loc).or_insert(0);
        *e = value;
    }

    let soln: u64 = dockmem.iter().map(|(_, v)| v).sum();
    println!("part 1 solution: {}", soln);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let instructions = parse_input_p2(input)?;
    let mut dockmem: HashMap<usize, u64> = HashMap::new();

    for instr in instructions {
        let n_write = 2usize.pow(instr.floating.len() as u32);

        for wix in 0..n_write {
            let mut target_loc = instr.loc as u64 | instr.set_mask;

            for ix in 0..instr.floating.len() {
                if wix & (1 << ix) != 0 {
                    target_loc |= 1 << instr.floating[ix];
                } else {
                    target_loc &= !(1 << instr.floating[ix]);
                }
            }

            let e = dockmem.entry(target_loc as usize).or_insert(0);
            *e = instr.value;
        }
    }

    let soln: u64 = dockmem.iter().map(|(_, v)| v).sum();
    println!("part 1 solution: {}", soln);
    Ok(())
}
