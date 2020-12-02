use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let start = std::time::Instant::now();
    part1(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    let start = std::time::Instant::now();
    part1_v2(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    let start = std::time::Instant::now();
    part2(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut validated_passwords = 0;

    for line in input.lines() {
        let lx = line.split_whitespace().take(3).collect::<Vec<&str>>();
        let mut p = lx[0].split("-").map(|s| s.parse().unwrap());
        let low: i32 = p.next().unwrap();
        let high: i32 = p.next().unwrap();

        let target = lx[1].trim().replace(":", "").parse::<char>()?;
        let password = lx[2].trim();

        let mut cnt = 0;

        for c in password.chars() {
            if c == target {
                cnt += 1;

                if cnt > high {
                    break;
                }
            }
        }

        if cnt >= low && cnt <= high {
            validated_passwords += 1;
        }
    }
    println!("part 1 solution: {}", validated_passwords);
    Ok(())
}

fn part1_v2(input: &str) -> Result<()> {
    let mut validated_passwords = 0;

    for line in input.lines() {
        let lx = line.split_whitespace().take(3).collect::<Vec<&str>>();
        let mut p = lx[0].split("-").map(|s| s.parse().unwrap());
        let low: usize = p.next().unwrap();
        let high: usize = p.next().unwrap();

        let target = lx[1].trim().replace(":", "").parse::<char>()?;
        let password = lx[2].trim();

        let cnt = password.matches(target).count();

        if cnt >= low && cnt <= high {
            validated_passwords += 1;
        }
    }
    println!("part 1 solution: {}", validated_passwords);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut validated_passwords = 0;

    for line in input.lines() {
        let lx = line.split_whitespace().take(3).collect::<Vec<&str>>();
        let mut p = lx[0].split("-").map(|s| s.parse::<usize>().unwrap());
        let p0: usize = p.next().unwrap() - 1;
        let p1: usize = p.next().unwrap() - 1;

        let target = lx[1].trim().replace(":", "").parse::<char>()?;
        let password = lx[2].trim();

        let mut cnt = 0;

        let a = password.chars().nth(p0).unwrap();
        let b = password.chars().nth(p1).unwrap();

        if a == target {
            cnt += 1
        }

        if b == target {
            cnt += 1
        }

        if cnt == 1 {
            validated_passwords += 1;
        }
    }
    println!("part 2 solution: {}", validated_passwords);
    Ok(())
}
