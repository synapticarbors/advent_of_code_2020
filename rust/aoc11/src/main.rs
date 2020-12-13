use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static MOVE_DIRS: [(isize, isize); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Debug)]
enum Part {
    P1,
    P2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatStatus {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

#[derive(Debug)]
struct Seat {
    status: SeatStatus,
    neighbors: Vec<usize>,
}

#[derive(Debug)]
struct WaitingRoom {
    nrows: usize,
    ncols: usize,
    seats: Vec<Seat>,
    curr_seat_status: Vec<SeatStatus>,
}

fn get_neighbors(nrows: usize, ncols: usize, idx: usize) -> Vec<usize> {
    let mut neighbors = vec![];

    let i = idx / ncols;
    let j = idx % ncols;

    for (mi, mj) in MOVE_DIRS.iter() {
        let ni = i as isize + mi;
        let nj = j as isize + mj;

        if (ni < 0) | (ni >= nrows as isize) | (nj < 0) | (nj >= ncols as isize) {
            continue;
        }

        let nidx = ni as usize * ncols + nj as usize;

        neighbors.push(nidx);
    }

    neighbors
}

struct DirScanner {
    idx: usize,
    dix: usize,
    m: usize,
    nrows: usize,
    ncols: usize,
}

impl DirScanner {
    fn new(nrows: usize, ncols: usize, idx: usize, dix: usize) -> DirScanner {
        DirScanner {
            idx: idx,
            dix: dix,
            m: 1,
            nrows: nrows,
            ncols: ncols,
        }
    }
}

impl Iterator for DirScanner {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let i = self.idx / self.ncols;
        let j = self.idx % self.ncols;

        let mi = MOVE_DIRS[self.dix].0 * self.m as isize;
        let mj = MOVE_DIRS[self.dix].1 * self.m as isize;

        let ni = i as isize + mi;
        let nj = j as isize + mj;

        self.m += 1;

        if (ni < 0) | (ni >= self.nrows as isize) | (nj < 0) | (nj >= self.ncols as isize) {
            None
        } else {
            Some(ni as usize * self.ncols + nj as usize)
        }
    }
}

impl WaitingRoom {
    fn from_input(input: &str) -> WaitingRoom {
        let mut sid = 0;

        // Get grid size
        let nrows = input.lines().count();
        let ncols = input
            .lines()
            .take(1)
            .next()
            .expect("could not extract first line")
            .chars()
            .count();

        let mut seats = Vec::with_capacity(nrows * ncols);
        let mut curr_seat_status = Vec::with_capacity(nrows * ncols);

        for line in input.lines() {
            for c in line.chars() {
                let status = match c {
                    '.' => SeatStatus::FLOOR,
                    '#' => SeatStatus::OCCUPIED,
                    'L' => SeatStatus::EMPTY,
                    _ => unreachable!(),
                };

                seats.push(Seat {
                    status: status.clone(),
                    neighbors: get_neighbors(nrows, ncols, sid),
                });

                curr_seat_status.push(status.clone());
                sid += 1;
            }
        }

        WaitingRoom {
            nrows,
            ncols,
            seats,
            curr_seat_status,
        }
    }

    fn print_grid(&self) {
        print!(
            "{}\n",
            self.curr_seat_status
                .chunks(self.ncols)
                .map(|x| {
                    x.iter()
                        .map(|s| match s {
                            SeatStatus::OCCUPIED => "#",
                            SeatStatus::EMPTY => "L",
                            SeatStatus::FLOOR => ".",
                        })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn get_num_visible_occupied(&self, idx: usize) -> i64 {
        let mut num_visible_occ = 0;

        for i in 0..8 {
            let ds = DirScanner::new(self.nrows, self.ncols, idx, i);

            for ni in ds {
                match self.seats[ni].status {
                    SeatStatus::OCCUPIED => {
                        num_visible_occ += 1;
                        break;
                    }
                    SeatStatus::EMPTY => break,
                    SeatStatus::FLOOR => (),
                }
                if let SeatStatus::OCCUPIED = self.seats[ni].status {
                    num_visible_occ += 1;
                    break;
                }
            }
        }

        num_visible_occ
    }

    fn get_seat_update_p1(&self, idx: usize) -> SeatStatus {
        let seat = &self.seats[idx];
        match seat.status {
            SeatStatus::EMPTY => {
                if seat
                    .neighbors
                    .iter()
                    .all(|&x| self.seats[x].status != SeatStatus::OCCUPIED)
                {
                    SeatStatus::OCCUPIED
                } else {
                    SeatStatus::EMPTY
                }
            }
            SeatStatus::FLOOR => SeatStatus::FLOOR,
            SeatStatus::OCCUPIED => {
                let num_occupied = seat
                    .neighbors
                    .iter()
                    .filter(|&x| self.seats[*x].status == SeatStatus::OCCUPIED)
                    .count();
                if num_occupied >= 4 {
                    SeatStatus::EMPTY
                } else {
                    SeatStatus::OCCUPIED
                }
            }
        }
    }

    fn get_seat_update_p2(&self, idx: usize) -> SeatStatus {
        let seat = &self.seats[idx];
        match seat.status {
            SeatStatus::EMPTY => {
                let noc = self.get_num_visible_occupied(idx);
                if noc == 0 {
                    SeatStatus::OCCUPIED
                } else {
                    SeatStatus::EMPTY
                }
            }
            SeatStatus::FLOOR => SeatStatus::FLOOR,
            SeatStatus::OCCUPIED => {
                let num_occupied = self.get_num_visible_occupied(idx);
                if num_occupied >= 5 {
                    SeatStatus::EMPTY
                } else {
                    SeatStatus::OCCUPIED
                }
            }
        }
    }

    fn update(&mut self, part: Part) -> bool {
        let mut room_changed = false;

        for (si, seat) in self.seats.iter().enumerate() {
            let new_status = match part {
                Part::P1 => self.get_seat_update_p1(si),
                Part::P2 => self.get_seat_update_p2(si),
            };
            if new_status != seat.status {
                room_changed = true;
                self.curr_seat_status[si] = new_status;
            }
        }

        if room_changed {
            for (seat, status) in self.seats.iter_mut().zip(self.curr_seat_status.iter()) {
                seat.status = *status;
            }
        }

        room_changed
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
    let mut room = WaitingRoom::from_input(input);

    while room.update(Part::P1) {
        continue;
    }

    let soln = room
        .seats
        .iter()
        .filter(|x| x.status == SeatStatus::OCCUPIED)
        .count();

    println!("part 1 solution: {}", soln);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut room = WaitingRoom::from_input(input);

    while room.update(Part::P2) {
        continue;
    }

    let soln = room
        .seats
        .iter()
        .filter(|x| x.status == SeatStatus::OCCUPIED)
        .count();

    println!("part 2 solution: {}", soln);

    Ok(())
}
