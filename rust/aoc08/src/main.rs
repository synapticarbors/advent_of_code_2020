use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, PartialEq)]
enum Ops {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

enum LoadStatus {
    COMPLETE,
    BUG,
}

#[derive(Debug, Default)]
struct BootProc {
    idx: i32,
    seen: HashSet<i32>,
    acc: i32,
}

impl BootProc {
    fn reset(&mut self) {
        self.idx = 0;
        self.seen.clear();
        self.acc = 0;
    }

    fn process_operations(&mut self, ops: &[Ops]) -> LoadStatus {
        loop {
            if !self.seen.insert(self.idx) {
                return LoadStatus::BUG;
            }

            if self.idx >= ops.len() as i32 {
                return LoadStatus::COMPLETE;
            }

            let o = ops.get(self.idx as usize).expect("operation out of bounds");
            match o {
                Ops::NOP(_) => self.idx += 1,
                Ops::JMP(x) => self.idx += x,
                Ops::ACC(x) => {
                    self.idx += 1;
                    self.acc += x
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Ops>> {
    let o = input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let op = iter.next().ok_or("")?;
            let num = iter.next().ok_or("")?.parse::<i32>()?;

            match op {
                "acc" => Ok(Ops::ACC(num)),
                "jmp" => Ok(Ops::JMP(num)),
                "nop" => Ok(Ops::NOP(num)),
                _ => unreachable!(),
            }
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(o)
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
    let operations = parse_input(input)?;
    let mut boot_proc: BootProc = Default::default();
    boot_proc.process_operations(&operations);

    println!("part 1 solution: {}", boot_proc.acc);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut operations = parse_input(input)?;
    let mut boot_proc: BootProc = Default::default();

    for i in 0..operations.len() {
        let o = &mut operations[i];
        let op_rep = match o {
            Ops::NOP(x) => Ops::JMP(*x),
            Ops::JMP(x) => Ops::NOP(*x),
            Ops::ACC(_) => continue,
        };

        boot_proc.reset();
        let opx = std::mem::replace(o, op_rep);

        if let LoadStatus::COMPLETE = boot_proc.process_operations(&operations) {
            println!("part 2 solution: {}", boot_proc.acc);
            return Ok(());
        }

        operations[i] = opx;
    }

    Err("part 2 NO SOLUTION FOUND".to_string().into())
}
