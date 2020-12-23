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
struct Game {
    current: usize,
    next_cup: Vec<usize>,
}

impl Game {
    fn build_game(nums: &[usize], pad_to: usize) -> Game {
        let current = nums[0];
        let sz: usize = if let Some(m) = nums.iter().max() {
            *m
        } else {
            unreachable!();
        };

        let addtl = (pad_to + 1) - (sz + 1);

        let mut next_cup = vec![0; pad_to + 1];

        for it in nums
            .iter()
            .cycle()
            .take(sz + 1)
            .copied()
            .collect::<Vec<usize>>()
            .windows(2)
        {
            next_cup[it[0]] = it[1]
        }

        if addtl > 0 {
            next_cup[nums[nums.len() - 1]] = sz + 1;
            for k in (sz + 1)..pad_to {
                next_cup[k] = k + 1;
            }

            next_cup[pad_to] = current;
        }

        Game { current, next_cup }
    }

    fn get_ordered_label(&self, start: usize) -> Vec<usize> {
        let mut x = vec![];
        x.push(start);

        let mut i = start;
        while self.next_cup[i] != start {
            let a = self.next_cup[i];
            x.push(a);
            i = a;
        }

        x
    }

    fn get_dest(&self, a: usize, b: usize, c: usize) -> usize {
        let mut t = self.current;
        let sz = self.next_cup.len() - 1;

        let mut x = (t + (sz - 1) - 1) % sz + 1;
        while (a == x) || (b == x) || (c == x) {
            t = x;
            x = (t + (sz - 1) - 1) % sz + 1;
        }

        x
    }

    fn play_turn(&mut self) {
        //
        let a = self.next_cup[self.current];
        let b = self.next_cup[a];
        let c = self.next_cup[b];

        let destination = self.get_dest(a, b, c);
        // move cups
        let cur_next = self.next_cup[c];
        let dest_next = self.next_cup[destination];

        self.next_cup[self.current] = cur_next;
        self.next_cup[destination] = a;
        self.next_cup[a] = b;
        self.next_cup[b] = c;
        self.next_cup[c] = dest_next;

        self.current = self.next_cup[self.current]
    }
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut x = Vec::new();

    for line in input.lines() {
        for c in line.chars() {
            let a = c.to_digit(10).unwrap() as usize;
            x.push(a)
        }
    }

    Ok(x)
}

fn part1(input: &str) -> Result<()> {
    let nums = parse_input(input)?;
    let mut game = Game::build_game(&nums, 9);

    for _ in 0..100 {
        game.play_turn();
    }

    let soln = game
        .get_ordered_label(1 as usize)
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("part 1 solution: {}", soln);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let nums = parse_input(input)?;
    let mut game = Game::build_game(&nums, 1_000_000);

    for _ in 0..10_000_000 {
        game.play_turn();
    }

    let a = game.next_cup[1];
    let b = game.next_cup[a];
    let soln = a * b;

    println!("part 2 solution: {}", soln);
    Ok(())
}
