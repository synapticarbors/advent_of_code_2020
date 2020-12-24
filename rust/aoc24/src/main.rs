use std::collections::HashMap;
use std::io::{self, Read};

use peg;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Excellent resource for hexagonal grids
// https://www.redblobgames.com/grids/hexagons

const DIR: [(isize, isize, isize); 6] = [
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
    let mut tiles = HashMap::new();

    for line in input.lines() {
        let moves = hextile_offset_parser::parse(line)?;

        let t = moves.iter().fold((0, 0, 0), |mut acc, off| {
            acc.0 += off.0;
            acc.1 += off.1;
            acc.2 += off.2;
            acc
        });

        let e = tiles.entry(t).or_insert(false);
        (*e) = !(*e);
    }

    let black_count = tiles.values().map(|&x| if x { 1 } else { 0 }).sum::<u64>();

    println!("part 1 solution: {}", black_count);
    Ok(())
}

type Tiles = HashMap<(isize, isize, isize), bool>;

fn flip(tile: &(isize, isize, isize), tiles: &Tiles) -> bool {
    let cnt = DIR.iter().fold(0, |mut acc, off| {
        match tiles
            .get(&(tile.0 + off.0, tile.1 + off.1, tile.2 + off.2))
            .unwrap_or(&false)
        {
            true => acc += 1,
            false => (),
        }

        acc
    });

    let status = tiles.get(tile).or(Some(&false)).unwrap();

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
    // find min max directions
    let mut x_max = std::isize::MIN;
    let mut x_min = std::isize::MAX;
    let mut y_max = std::isize::MIN;
    let mut y_min = std::isize::MAX;
    let mut z_max = std::isize::MIN;
    let mut z_min = std::isize::MAX;

    for t in tiles.keys() {
        if t.0 > x_max {
            x_max = t.0
        } else if t.0 < x_min {
            x_min = t.0
        }

        if t.1 > y_max {
            y_max = t.1
        } else if t.1 < y_min {
            y_min = t.1
        }

        if t.2 > z_max {
            z_max = t.2
        } else if t.2 < z_min {
            z_min = t.2
        }
    }

    let mut changes = Vec::new();
    for x in x_min - 1..x_max + 2 {
        for y in y_min - 1..y_max + 2 {
            for z in z_min - 1..z_max + 2 {
                let tc = (x, y, z);
                let do_flip = flip(&tc, tiles);

                if do_flip {
                    changes.push((tc, do_flip));
                }
            }
        }
    }

    for (tile, do_flip) in changes.iter() {
        let e = tiles.entry(*tile).or_insert(false);
        if *do_flip {
            (*e) = !(*e);
        }
    }
}

fn part2(input: &str) -> Result<()> {
    let mut tiles = HashMap::new();

    // Get initial layout
    for line in input.lines() {
        let moves = hextile_offset_parser::parse(line)?;

        let t = moves.iter().fold((0, 0, 0), |mut acc, off| {
            acc.0 += off.0;
            acc.1 += off.1;
            acc.2 += off.2;
            acc
        });

        let e = tiles.entry(t).or_insert(false);
        (*e) = !(*e);
    }

    // living art
    for _ in 0..100 {
        update_day(&mut tiles);
    }

    let black_count = tiles.values().map(|&x| if x { 1 } else { 0 }).sum::<u64>();
    println!("part 2 solution: {}", black_count);

    Ok(())
}
