use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct TicketDecoder {
    pub row_low: u8,
    pub row_high: u8,
    pub col_low: u8,
    pub col_high: u8,
}

fn midpoint(low: u8, high: u8) -> u8 {
    (low & high) + ((low ^ high) >> 1)
}

impl TicketDecoder {
    fn new() -> TicketDecoder {
        TicketDecoder {
            row_low: 0,
            row_high: 127,
            col_low: 0,
            col_high: 7,
        }
    }

    fn reset(&mut self) {
        self.row_low = 0;
        self.row_high = 127;
        self.col_low = 0;
        self.col_high = 7;
    }

    fn find_seat(&mut self, seq: &str) -> (u8, u8) {
        for (i, c) in seq.chars().enumerate() {
            match i {
                0..=6 => {
                    let m = midpoint(self.row_low, self.row_high);
                    match c {
                        'F' => self.row_high = m,
                        'B' => self.row_low = m + 1,
                        _ => unreachable!(),
                    }
                }
                7..=9 => {
                    let m = midpoint(self.col_low, self.col_high);
                    match c {
                        'R' => self.col_low = m + 1,
                        'L' => self.col_high = m,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        (self.row_low, self.col_low)
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
    let mut max_seat_id = 0;
    let mut seat_decoder = TicketDecoder::new();

    for line in input.lines() {
        seat_decoder.reset();
        let (r, c) = seat_decoder.find_seat(line);
        let seat_id = (r as u32) * 8 + (c as u32);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    println!("part 1 solution: {}", max_seat_id);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut seat_decoder = TicketDecoder::new();

    let seat_ids: Vec<u32> = input
        .lines()
        .map(|line| {
            seat_decoder.reset();
            let (r, c) = seat_decoder.find_seat(line);
            (r as u32) * 8 + (c as u32)
        })
        .collect();

    // For a list of N consecutive numbers the sum of 1 to N inclusive
    // is N * (N + 1 ) / 2. Here we calculate the sum if there were no
    // missing numbers between 0 and the max seat id and then subtract
    // the sum of those missing in the front as well as the seat ids that
    // we observe, leaving the missing seat id.

    let mut seat_min = u32::MAX;
    let mut seat_max = u32::MIN;
    let mut s = 0;

    for &x in seat_ids.iter() {
        if x < seat_min {
            seat_min = x;
        }
        if x > seat_max {
            seat_max = x;
        }
        s += x;
    }

    let total_front = {
        let x = seat_min - 1;
        x * (x + 1) / 2
    };

    let total_back = seat_max * (seat_max + 1) / 2;

    let missing = total_back - total_front - s;

    println!("part 2 solution: {}", missing);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut seat_decoder = TicketDecoder::new();

        let (r, c) = seat_decoder.find_seat("BFFFBBFRRR");
        assert!(r == 70);
        assert!(c == 7);
        seat_decoder.reset();

        let (r, c) = seat_decoder.find_seat("FFFBBBFRRR");
        assert!(r == 14);
        assert!(c == 7);
        seat_decoder.reset();

        let (r, c) = seat_decoder.find_seat("BBFFBBFRLL");
        assert!(r == 102);
        assert!(c == 4);
        seat_decoder.reset();
    }
}
