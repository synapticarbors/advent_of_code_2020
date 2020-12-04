use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct PassportBatch {
    pub passports: Vec<Passport>,
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn default() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    fn from_map(d: &HashMap<String, String>) -> Result<Passport> {
        let mut pp = Passport::default();
        for (k, v) in d {
            match k.as_ref() {
                "byr" => pp.byr = Some(v.parse()?),
                "iyr" => pp.iyr = Some(v.parse()?),
                "eyr" => pp.eyr = Some(v.parse()?),
                "hgt" => pp.hgt = Some(v.parse()?),
                "hcl" => pp.hcl = Some(v.parse()?),
                "ecl" => pp.ecl = Some(v.parse()?),
                "pid" => pp.pid = Some(v.parse()?),
                "cid" => pp.cid = Some(v.parse()?),
                _ => (),
            }
        }

        Ok(pp)
    }

    fn is_byr_valid(&self) -> bool {
        if let Some(x) = self
            .byr
            .as_ref()
            .and_then(|v| v.parse::<i16>().ok())
            .and_then(|x| Some((x >= 1920) & (x <= 2002)))
        {
            x
        } else {
            false
        }
    }

    fn is_iyr_valid(&self) -> bool {
        if let Some(x) = self
            .iyr
            .as_ref()
            .and_then(|v| v.parse::<i16>().ok())
            .and_then(|x| Some((x >= 2010) & (x <= 2020)))
        {
            x
        } else {
            false
        }
    }

    fn is_eyr_valid(&self) -> bool {
        if let Some(x) = self
            .eyr
            .as_ref()
            .and_then(|v| v.parse::<i16>().ok())
            .and_then(|x| Some((x >= 2020) & (x <= 2030)))
        {
            x
        } else {
            false
        }
    }

    fn is_hgt_valid(&self) -> bool {
        if let Some(x) = self
            .hgt
            .as_ref()
            .and_then(|v| {
                let is_cm = v.contains("cm");
                let is_in = v.contains("in");
                let vs = v.replace("cm", "").replace("in", "").parse::<i16>();
                Some((is_cm, is_in, vs))
            })
            .and_then(|x| match x {
                (true, false, Ok(y)) => Some((y >= 150) & (y <= 193)),
                (false, true, Ok(y)) => Some((y >= 59) & (y <= 76)),
                _ => Some(false),
            })
        {
            x
        } else {
            false
        }
    }

    fn is_hcl_valid(&self) -> bool {
        if let Some(x) = self.hcl.as_ref().and_then(|v| {
            let corr_len = v.len() == 7;
            let corr_set = v.chars().skip(1).all(|x| "0123456789abcdef".contains(x));
            Some(corr_len & corr_set)
        }) {
            x
        } else {
            false
        }
    }

    fn is_ecl_valid(&self) -> bool {
        let valid_ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if let Some(x) = self.ecl.as_ref() {
            valid_ecls.contains(&x.as_ref())
        } else {
            false
        }
    }

    fn is_pid_valid(&self) -> bool {
        if let Some(x) = self.pid.as_ref() {
            let corr_len = x.len() == 9;
            let is_num = x.parse::<i32>().is_ok();

            corr_len & is_num
        } else {
            false
        }
    }

    fn is_valid_part1(&self) -> bool {
        self.byr.is_some()
            & self.iyr.is_some()
            & self.eyr.is_some()
            & self.hgt.is_some()
            & self.hcl.is_some()
            & self.ecl.is_some()
            & self.pid.is_some()
    }

    fn is_valid_part2(&self) -> bool {
        self.is_byr_valid()
            & self.is_iyr_valid()
            & self.is_eyr_valid()
            & self.is_hgt_valid()
            & self.is_hcl_valid()
            & self.is_ecl_valid()
            & self.is_pid_valid()
    }
}

fn parse_kv_pair(data: &str) -> (String, String) {
    let mut x = data.split(":");
    let k = x.next().unwrap().to_string();
    let v = x.next().unwrap().to_string();

    (k, v)
}

impl PassportBatch {
    fn from_file(data: &str) -> PassportBatch {
        let vx = data.lines().fold(vec![HashMap::new()], |mut acc, v| {
            if v.is_empty() {
                acc.push(HashMap::new());
                acc
            } else {
                v.split_whitespace().for_each(|g| {
                    let (k, v) = parse_kv_pair(g);
                    if let Some(e) = acc.last_mut() {
                        e.insert(k, v);
                    }
                });
                acc
            }
        });

        let v = vx.iter().map(|d| Passport::from_map(d).unwrap()).collect();

        PassportBatch { passports: v }
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
    let batch = PassportBatch::from_file(input);
    let n_valid = batch
        .passports
        .iter()
        .filter(|x| x.is_valid_part1())
        .count();

    println!("part 1 solution: {}", n_valid);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let batch = PassportBatch::from_file(input);
    let n_valid = batch
        .passports
        .iter()
        .filter(|x| x.is_valid_part2())
        .count();

    println!("part 2 solution: {}", n_valid);
    Ok(())
}
