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

fn part1(input: &str) -> Result<()> {
    let tree = "#".chars().nth(0);
    let num_trees = input
        .lines()
        .skip(1)
        .enumerate()
        .map(|(i, l)| l.chars().nth(3 * (i + 1) % l.len()))
        .filter(|x| x == &tree)
        .count();

    println!("part 1 solution: {}", num_trees);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let tree = "#".chars().nth(0);
    let paths = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let soln: usize = paths
        .iter()
        .map(|(st_right, st_down)| {
            input
                .lines()
                .skip(*st_down as usize)
                .step_by(*st_down as usize)
                .enumerate()
                .map(|(i, l)| l.chars().nth((st_right * (i + 1)) % l.len()))
                .filter(|x| x == &tree)
                .count()
        })
        .product();

    println!("part 2 solution: {:?}", soln);

    Ok(())
}
