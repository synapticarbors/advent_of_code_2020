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

fn parse_input(input: &str) -> Result<(i32, Vec<i32>)> {
    let mut it = input.lines();
    let target = it.next().ok_or("no target")?.parse::<i32>()?;
    let sched = it
        .next()
        .ok_or("no schedule")?
        .split(",")
        .map(|x| match x {
            "x" => -1,
            _ => x.parse::<i32>().unwrap(),
        })
        .collect::<Vec<_>>();

    Ok((target, sched))
}

fn part1(input: &str) -> Result<()> {
    let (target, sched) = parse_input(input)?;

    let mut best = i32::MAX;
    let mut best_id = None;

    for v in sched {
        if v == -1 {
            continue;
        }

        if target % v == 0 {
            best = target;
            best_id = Some(v);
            break;
        }

        let x = (target as f32 / v as f32).ceil() as i32 * v;

        if x < best {
            best = x;
            best_id = Some(v);
        }
    }

    if let Some(bus_id) = best_id {
        println!("part 1 solution: {}", bus_id * (best - target));
    } else {
        println!("No solution found");
    }

    Ok(())
}

fn scan(sched: &[i32]) -> i64 {
    let mut time: i64 = 0;
    let mut step: i64 = 1;

    for (offset, bus_id) in sched.iter().map(|&x| x as i64).enumerate() {
        if bus_id == -1 {
            continue;
        }

        while (time + offset as i64) % bus_id != 0 {
            time += step;
        }

        step *= bus_id;
    }

    time
}

fn part2(input: &str) -> Result<()> {
    let (_, sched) = parse_input(input)?;

    let soln = scan(&sched);

    println!("part 2 solution: {}", soln);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let sched = vec![17, -1, 13, 19];
        assert!(scan(&sched) == 3417);

        let sched = vec![67, 7, 59, 61];
        assert!(scan(&sched) == 754018);

        let sched = vec![67, -1, 7, 59, 61];
        assert!(scan(&sched) == 779210);

        let sched = vec![67, 7, -1, 59, 61];
        assert!(scan(&sched) == 1261476);

        let sched = vec![1789, 37, 47, 1889];
        assert!(scan(&sched) == 1202161486);
    }
}
