use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod part2;

static MOVE_DIRS: [(isize, isize, isize); 26] = [
    (-1, -1, -1),
    (-1, -1, 1),
    (-1, -1, 0),
    (-1, 1, -1),
    (-1, 1, 1),
    (-1, 1, 0),
    (-1, 0, -1),
    (-1, 0, 1),
    (-1, 0, 0),
    (1, -1, -1),
    (1, -1, 1),
    (1, -1, 0),
    (1, 1, -1),
    (1, 1, 1),
    (1, 1, 0),
    (1, 0, -1),
    (1, 0, 1),
    (1, 0, 0),
    (0, -1, -1),
    (0, -1, 1),
    (0, -1, 0),
    (0, 1, -1),
    (0, 1, 1),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

type Cube = (isize, isize, isize);

#[derive(Debug, Default)]
struct Grid {
    active: HashSet<Cube>,
    update: HashSet<Cube>,
}

impl Grid {
    fn from_input(input: &str) -> Result<Grid> {
        let mut grid: Grid = Default::default();

        let mut x: isize = 0;
        let mut y: isize = 0;

        for line in input.lines() {
            for c in line.chars() {
                match c {
                    '#' => {
                        grid.active.insert((x, y, 0));
                    }
                    '.' => (),
                    _ => unreachable!(),
                }

                x += 1;
            }
            x = 0;
            y += 1;
        }

        Ok(grid)
    }

    fn get_active_neighbors_count(&self, p: &Cube) -> i16 {
        let mut cnt = 0;

        for (mx, my, mz) in MOVE_DIRS.iter() {
            let xn = p.0 + mx;
            let yn = p.1 + my;
            let zn = p.2 + mz;

            if self.active.contains(&(xn, yn, zn)) {
                cnt += 1
            }

            if cnt > 3 {
                break;
            }
        }

        cnt
    }

    fn get_min_max(&self) -> (Cube, Cube) {
        let mut min: Cube = (std::isize::MAX, std::isize::MAX, std::isize::MAX);
        let mut max: Cube = (std::isize::MIN, std::isize::MIN, std::isize::MIN);

        for x in self.active.iter() {
            if x.0 < min.0 {
                min.0 = x.0;
            }

            if x.1 < min.1 {
                min.1 = x.1;
            }

            if x.2 < min.2 {
                min.2 = x.2;
            }

            if x.0 > max.0 {
                max.0 = x.0;
            }

            if x.1 > max.1 {
                max.1 = x.1;
            }

            if x.2 > max.2 {
                max.2 = x.2;
            }
        }

        (min, max)
    }

    fn update(&mut self) {
        let (min_coords, max_coords) = self.get_min_max();

        for xt in min_coords.0 - 1..max_coords.0 + 2 {
            for yt in min_coords.1 - 1..max_coords.1 + 2 {
                for zt in min_coords.2 - 1..max_coords.2 + 2 {
                    let cube: Cube = (xt, yt, zt);
                    let ncnt = self.get_active_neighbors_count(&cube);
                    if self.active.contains(&cube) {
                        if (ncnt == 2) | (ncnt == 3) {
                            self.update.insert(cube);
                        }
                    } else {
                        if ncnt == 3 {
                            self.update.insert(cube);
                        }
                    }
                }
            }
        }

        self.active = self.update.drain().collect();
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
    let mut grid = Grid::from_input(input)?;

    for _ in 0..6 {
        grid.update();
    }

    println!("part 1 solution: {}", grid.active.len());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    // Didn't feel like making the problem generic over the number of
    // dimensions so just wholesale copy the code into new module.
    // Lazy, I know.
    part2::part2(input)?;
    Ok(())
}
