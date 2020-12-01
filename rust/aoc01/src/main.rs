use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut values = vec![];

    for line in input.lines() {
        let v: i32 = line.parse()?;
        values.push(v);
    }

    let n = values.len();

    for i in 0..n - 1 {
        for j in i + 1..n {
            let x = values[i];
            let y = values[j];

            if x + y == 2020 {
                println!("part 1 solution: {}", x * y);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut values = vec![];

    for line in input.lines() {
        let v: i32 = line.parse()?;
        values.push(v);
    }

    let n = values.len();
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let x = values[i];
                let y = values[j];
                let z = values[k];

                if x + y + z == 2020 {
                    println!("part 2 solution: {}", x * y * z);
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}
