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

fn find_loop_size(public_key: u64) -> u64 {
    let mut loop_size = 0;
    let mut value = 1;

    while value != public_key {
        value *= 7;
        value %= 20201227;

        loop_size += 1;
    }

    loop_size
}

fn get_enc_key(loop_size: u64, key: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= key;
        value %= 20201227;
    }

    value
}

fn part1(input: &str) -> Result<()> {
    let public_keys: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();

    let card_loop_size = find_loop_size(public_keys[0]);
    let door_loop_size = find_loop_size(public_keys[1]);

    let card_enc = get_enc_key(door_loop_size, public_keys[0]);
    let door_enc = get_enc_key(card_loop_size, public_keys[1]);

    println!(
        "part 1 solution: {} {} (should be equal; pick one)",
        card_enc, door_enc
    );

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);

        assert_eq!(get_enc_key(11, 5764801), 14897079);
        assert_eq!(get_enc_key(8, 17807724), 14897079);
    }
}
