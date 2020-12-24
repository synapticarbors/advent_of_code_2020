use std::collections::HashSet;
use std::io::{self, Read};

use peg;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Excellent resource for hexagonal grids
// https://www.redblobgames.com/grids/hexagons

const OFFSETS: [(isize, isize, isize); 6] = [
    (1, -1, 0),
    (-1, 1, 0),
    (0, 1, -1),
    (1, 0, -1),
    (-1, 0, 1),
    (0, -1, 1),
];

peg::parser! {
    grammar hextile_offset_parser() for str {
        rule east() -> (isize, isize, isize)
            = x:$("e") { (1, -1, 0) }

        rule west() -> (isize, isize, isize)
            = x:$("w") { (-1, 1, 0) }

        rule northwest() -> (isize, isize, isize)
            = x:$("nw") { (0, 1, -1) }

        rule northeast() -> (isize, isize, isize)
            = x:$("ne") { (1, 0, -1) }

        rule southwest() -> (isize, isize, isize)
            = x:$("sw") { (-1, 0, 1) }

        rule southeast() -> (isize, isize, isize)
            = x:$("se") { (0, -1, 1) }

        pub rule parse() -> Vec<(isize, isize, isize)>
            = n:(northeast() / northwest() / southeast() / southwest() / east() / west()) ++ "" { n }

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
    let mut tiles = HashSet::new();

    for line in input.lines() {
        let moves = hextile_offset_parser::parse(line)?;

        let t = moves.iter().fold((0, 0, 0), |mut acc, off| {
            acc.0 += off.0;
            acc.1 += off.1;
            acc.2 += off.2;
            acc
        });

        if !tiles.remove(&t) {
            tiles.insert(t);
        }
    }

    let black_count = tiles.len();

    println!("part 1 solution: {}", black_count);
    Ok(())
}

type Tiles = HashSet<(isize, isize, isize)>;

fn flip(tile: &(isize, isize, isize), tiles: &Tiles) -> bool {
    let cnt = OFFSETS.iter().fold(0, |mut acc, off| {
        if tiles.contains(&(tile.0 + off.0, tile.1 + off.1, tile.2 + off.2)) {
            acc += 1
        }

        acc
    });

    let status = tiles.contains(tile);

    match status {
        true => match cnt {
            0 => true,
            1 | 2 => false,
            _ => true,
        },
        false => match cnt {
            2 => true,
            _ => false,
        },
    }
}

fn update_day(tiles: &mut Tiles) {
    let mut tile_candidates = HashSet::new();

    tile_candidates.extend(tiles.iter());

    for tile in tiles.iter() {
        for d in OFFSETS.iter() {
            tile_candidates.insert((tile.0 + d.0, tile.1 + d.1, tile.2 + d.2));
        }
    }

    let mut changes = Vec::new();
    for tc in tile_candidates.iter() {
        let do_flip = flip(&tc, tiles);

        if do_flip {
            changes.push(*tc);
        }
    }

    for tile in changes.iter() {
        if !tiles.remove(tile) {
            tiles.insert(*tile);
        }
    }
}

fn part2(input: &str) -> Result<()> {
    let mut tiles = HashSet::new();

    for line in input.lines() {
        let moves = hextile_offset_parser::parse(line)?;

        let t = moves.iter().fold((0, 0, 0), |mut acc, off| {
            acc.0 += off.0;
            acc.1 += off.1;
            acc.2 += off.2;
            acc
        });

        if !tiles.remove(&t) {
            tiles.insert(t);
        }
    }

    // living art
    for _ in 0..100 {
        update_day(&mut tiles);
    }

    let black_count = tiles.len();
    println!("part 2 solution: {}", black_count);

    Ok(())
}
