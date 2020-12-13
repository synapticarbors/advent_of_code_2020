use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// N = 0; E = 1;, S = 2; W = 3
#[derive(Debug)]
struct Path {
    curr_direction: i32,
    dists: [i32; 4],
}

impl Path {
    fn new(init_direction: i32) -> Path {
        Path {
            curr_direction: init_direction,
            dists: [0; 4],
        }
    }

    fn dist_moved(&self) -> i32 {
        (self.dists[0] - self.dists[2]).abs() + (self.dists[1] - self.dists[3]).abs()
    }
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
    let p = input.lines().fold(Path::new(1), |mut acc, line| {
        let move_dir = &line[..1];
        let num = &line[1..].parse::<i32>().unwrap();

        //println!("{} {} {:?}", move_dir, num, acc);
        match move_dir {
            "N" => acc.dists[0] += num,
            "S" => acc.dists[2] += num,
            "E" => acc.dists[1] += num,
            "W" => acc.dists[3] += num,
            "F" => acc.dists[acc.curr_direction as usize] += num,
            "L" => acc.curr_direction = (acc.curr_direction - (num / 90)).rem_euclid(4),
            "R" => acc.curr_direction = (acc.curr_direction + (num / 90)).rem_euclid(4),
            _ => unreachable!(),
        }

        acc
    });
    println!("part 1 solution: {}", p.dist_moved());
    Ok(())
}

#[derive(Debug)]
struct Ship {
    pos: [i32; 2],
    wayp: [i32; 2],
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos: [0; 2],
            wayp: [10, 1],
        }
    }

    fn dist_moved(&self) -> i32 {
        self.pos[0].abs() + self.pos[1].abs()
    }
}

fn part2(input: &str) -> Result<()> {
    let p = input.lines().fold(Ship::new(), |mut acc, line| {
        let action = &line[..1];
        let num = &line[1..].parse::<i32>().unwrap();

        match action {
            "N" => acc.wayp[1] += num,
            "S" => acc.wayp[1] -= num,
            "E" => acc.wayp[0] += num,
            "W" => acc.wayp[0] -= num,
            "F" => {
                acc.pos[0] += num * acc.wayp[0];
                acc.pos[1] += num * acc.wayp[1];
            }
            "R" | "L" => {
                let s: i32 = if action == "R" { 1 } else { -1 };
                let (a, b, c, d) = match num {
                    90 => (0, s, -s, 0),
                    180 => (-1, 0, 0, -1),
                    270 => (0, -s, s, 0),
                    _ => unreachable!(),
                };
                let x = a * acc.wayp[0] + b * acc.wayp[1];
                let y = c * acc.wayp[0] + d * acc.wayp[1];
                acc.wayp[0] = x;
                acc.wayp[1] = y;
            }
            _ => unreachable!(),
        }

        acc
    });
    println!("part 2 solution: {}", p.dist_moved());
    Ok(())
}
