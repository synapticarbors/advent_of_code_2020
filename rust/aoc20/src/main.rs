use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::FromIterator;

use peg;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

peg::parser! {
    grammar tile_parser() for str {
        rule num() -> u32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule element() -> bool
            = x:$(['#' | '.']) {
                let c = x.chars().next().unwrap();
                match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!()
                }
            }

        rule tile_line() -> u32
            = "Tile" _ n:num() ":" { n }

        rule piece_line() -> String
            = x:$(['#' | '.']+) { x.replace('#', "1").replace('.', "0") }

        rule _() = " "?

        pub rule parse() -> (u32, Vec<String>)
            = n:tile_line() "\n"  x:piece_line() ++ "\n" { (n, x) }
    }
}

#[derive(Debug)]
struct Edges {
    top: String,
    bottom: String,
    left: String,
    right: String,
}

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

impl Edges {
    fn from_raw(r: &[String]) -> Edges {
        let nrows = r.len();
        Edges {
            top: r[0].to_owned(),
            bottom: r[nrows - 1].clone(),
            left: r
                .iter()
                .map(|x| x.chars().nth(0).unwrap())
                .collect::<String>(),
            right: r
                .iter()
                .map(|x| x.chars().last().unwrap())
                .collect::<String>(),
        }
    }

    fn to_view(&self) -> Result<[u16; 4]> {
        let top = u16::from_str_radix(&self.top, 2)?;
        let bottom = u16::from_str_radix(&self.bottom, 2)?;
        let left = u16::from_str_radix(&self.left, 2)?;
        let right = u16::from_str_radix(&self.right, 2)?;

        Ok([top, right, bottom, left])
    }

    fn rotate(&self) -> Edges {
        Edges {
            top: self.right.clone(),
            bottom: self.left.clone(),
            left: self.top.chars().rev().collect(),
            right: self.bottom.chars().rev().collect(),
        }
    }

    fn flip(&self) -> Edges {
        Edges {
            top: self.top.chars().rev().collect(),
            bottom: self.bottom.chars().rev().collect(),
            left: self.right.clone(),
            right: self.left.clone(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Tile {
    tid: u32,
    views: [[u16; 4]; 8],
    raw_image: Vec<String>,
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

#[derive(Debug)]
struct PlacedTile<'a> {
    tile: &'a Tile,
    oid: usize,
}

fn parse_input(input: &str) -> Result<Vec<Tile>> {
    let it = input.split("\n\n");

    let mut tiles = vec![];

    for g in it {
        let (tid, raw_image) = tile_parser::parse(g)?;

        let e0 = Edges::from_raw(&raw_image);
        let e1 = e0.rotate();
        let e2 = e1.rotate();
        let e3 = e2.rotate();
        let e4 = e0.flip();
        let e5 = e4.rotate();
        let e6 = e5.rotate();
        let e7 = e6.rotate();

        tiles.push(Tile {
            tid: tid,
            views: [
                e0.to_view()?,
                e1.to_view()?,
                e2.to_view()?,
                e3.to_view()?,
                e4.to_view()?,
                e5.to_view()?,
                e6.to_view()?,
                e7.to_view()?,
            ],
            raw_image: raw_image,
        });
    }

    Ok(tiles)
}

fn find_corners(tiles: &[Tile]) -> Vec<u32> {
    // Create HashMap of edges and the tiles that contain that edge. Tiles
    // with an unmatched edge have to be along the border.
    let mut cnt: HashMap<u16, HashSet<u32>> = HashMap::new();
    for tile in tiles {
        let tid = tile.tid;
        let v = &tile.views;

        for s in v.iter().flatten() {
            let e = cnt.entry(*s).or_insert(HashSet::new());
            (*e).insert(tid);
        }
    }

    let mut tile_unmatched_edges: HashMap<u32, HashSet<u16>> = HashMap::new();

    for (en, tids) in &mut cnt {
        if tids.len() == 1 {
            if let Some(tidx) = tids.iter().next() {
                let e = tile_unmatched_edges
                    .entry(*tidx)
                    .or_insert(HashSet::with_capacity(4));
                (*e).insert(*en);
            }
        }
    }

    // Find tiles that have two unmatched edges. These are at the corners
    // Since the flipped edges will also be unmatched, there will be 4 edges all together
    let mut corner_tiles = vec![];

    for (tidx, edges) in &tile_unmatched_edges {
        if edges.len() == 4 {
            corner_tiles.push(*tidx);
        }
    }

    corner_tiles
}

fn get_starting_orientation(t: &Tile, tiles: &[Tile]) -> usize {
    let mut target_oid = None;
    let mut other_en: HashSet<u16> = HashSet::new();

    for oid in 0..8 {
        let a = t.views[oid][RIGHT];
        let b = t.views[oid][BOTTOM];

        other_en.clear();

        for ot in tiles.iter() {
            if ot.tid == t.tid {
                continue;
            }
            let v = &ot.views;

            for s in v.iter().flatten() {
                other_en.insert(*s);
            }
        }

        if other_en.contains(&a) & other_en.contains(&b) {
            target_oid = Some(oid);
            break;
        }
    }

    if let Some(x) = target_oid {
        return x;
    }
    unreachable!();
}

fn solve_puzzle(tiles: &[Tile]) -> HashMap<(u8, u8), PlacedTile> {
    let ntiles = tiles.len();
    let psize = (ntiles as f64).sqrt() as u32;
    let corner_tiles_ids = find_corners(&tiles);

    let mut available_tiles: HashSet<&Tile> = HashSet::from_iter(tiles.iter());

    let mut solution = HashMap::with_capacity(ntiles);

    // Pick random corner piece in original orientation to start with
    let t = tiles
        .iter()
        .filter(|&x| x.tid == corner_tiles_ids[0])
        .nth(0)
        .unwrap();

    let starting_oid = get_starting_orientation(t, tiles);

    solution.insert(
        (0 as u8, 0 as u8),
        PlacedTile {
            tile: &t,
            oid: starting_oid,
        },
    );

    available_tiles.remove(&t);

    for i in 0..psize {
        for j in 0..psize {
            if (i, j) == (0, 0) {
                continue;
            }

            let left_neighbor = {
                if j == 0 {
                    None
                } else {
                    Some(solution.get(&(i as u8, j as u8 - 1)).unwrap())
                }
            };

            let up_neighbor = {
                if i == 0 {
                    None
                } else {
                    Some(solution.get(&(i as u8 - 1, j as u8)).unwrap())
                }
            };

            let (matched_tile, matched_oid) = match (left_neighbor, up_neighbor) {
                (Some(l_pt), None) => {
                    let target_left = l_pt.tile.views[l_pt.oid][RIGHT];
                    let mut mt = None;
                    let mut mt_oid = None;

                    'outer1: for candidate_tile in available_tiles.iter() {
                        for oid in 0..8 {
                            if candidate_tile.views[oid][LEFT] == target_left {
                                mt = Some(*candidate_tile);
                                mt_oid = Some(oid);
                                break 'outer1;
                            }
                        }
                    }

                    (mt, mt_oid)
                }
                (Some(l_pt), Some(u_pt)) => {
                    let target_left = l_pt.tile.views[l_pt.oid][RIGHT];
                    let target_up = u_pt.tile.views[u_pt.oid][BOTTOM];

                    let mut mt = None;
                    let mut mt_oid = None;

                    'outer2: for candidate_tile in available_tiles.iter() {
                        for oid in 0..8 {
                            if (candidate_tile.views[oid][TOP] == target_up)
                                & (candidate_tile.views[oid][LEFT] == target_left)
                            {
                                mt = Some(*candidate_tile);
                                mt_oid = Some(oid);
                                break 'outer2;
                            }
                        }
                    }

                    (mt, mt_oid)
                }
                (None, Some(u_pt)) => {
                    let target_up = u_pt.tile.views[u_pt.oid][BOTTOM];
                    let mut mt = None;
                    let mut mt_oid = None;

                    'outer3: for candidate_tile in available_tiles.iter() {
                        for oid in 0..8 {
                            if candidate_tile.views[oid][TOP] == target_up {
                                mt = Some(*candidate_tile);
                                mt_oid = Some(oid);
                                break 'outer3;
                            }
                        }
                    }

                    (mt, mt_oid)
                }
                (None, None) => unreachable!(),
            };

            match (matched_tile, matched_oid) {
                (Some(mt), Some(mt_oid)) => {
                    solution.insert(
                        (i as u8, j as u8),
                        PlacedTile {
                            tile: &mt,
                            oid: mt_oid,
                        },
                    );
                    available_tiles.remove(&mt);
                }
                _ => unreachable!(),
            }
        }
    }

    solution
}

fn part1(input: &str) -> Result<()> {
    let tiles = parse_input(input)?;
    let corner_tiles_ids = find_corners(&tiles);

    println!(
        "part 1 solution: {}",
        corner_tiles_ids.iter().map(|&x| x as u64).product::<u64>()
    );

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let tiles = parse_input(input)?;
    let solved = solve_puzzle(&tiles);

    Ok(())
}
