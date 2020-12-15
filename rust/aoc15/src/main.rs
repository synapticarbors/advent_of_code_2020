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

fn parse_input(input: &str) -> Result<Vec<u32>> {
    let mut out = vec![];
    for line in input.lines() {
        for x in line.split(",") {
            out.push(x.parse::<u32>()?);
        }
    }
    Ok(out)
}

#[derive(Debug)]
struct GameData {
    last_seen: HashMap<u32, u32>,
    turn: u32,
    last_spoken: u32,
}

impl GameData {
    fn new() -> GameData {
        GameData {
            last_seen: HashMap::new(),
            turn: 1,
            last_spoken: 0,
        }
    }

    fn play_turn(&mut self) {
        if let Some(prev_turn) = self.last_seen.insert(self.last_spoken, self.turn - 1) {
            self.last_spoken = self.turn - 1 - prev_turn;
        } else {
            self.last_spoken = 0;
        }

        self.turn += 1;
    }
}

fn play_game(start_numbers: &[u32], max_turns: u32) -> u32 {
    let mut gd = GameData::new();

    for num in start_numbers {
        gd.last_seen.insert(*num, gd.turn);
        gd.last_spoken = *num;
        gd.turn += 1;
    }

    while gd.turn <= max_turns {
        gd.play_turn();
        //println!("{}, {}", gd.turn - 1, gd.last_spoken);
    }

    gd.last_spoken
}

fn part1(input: &str) -> Result<()> {
    let start_numbers = parse_input(input)?;

    let soln = play_game(&start_numbers, 2020);
    println!("part 1 solution: {}", soln);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let start_numbers = parse_input(input)?;

    let soln = play_game(&start_numbers, 30000000);
    println!("part 1 solution: {}", soln);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let start_numbers = vec![0, 3, 6];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 436);

        let start_numbers = vec![1, 3, 2];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 1);

        let start_numbers = vec![2, 1, 3];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 10);

        let start_numbers = vec![1, 2, 3];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 27);

        let start_numbers = vec![2, 3, 1];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 78);

        let start_numbers = vec![3, 2, 1];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 438);

        let start_numbers = vec![3, 1, 2];
        let soln = play_game(&start_numbers, 2020);
        assert!(soln == 1836);
    }

    #[test]
    fn part2_test() {
        let start_numbers = vec![0, 3, 6];
        let soln = play_game(&start_numbers, 30000000);
        assert!(soln == 175594);
    }
}
