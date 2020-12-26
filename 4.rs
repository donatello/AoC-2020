use std::convert::TryInto;
use std::io::{self, BufRead};

// Leave out "cid".
const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn mk_passports(v: Vec<String>) -> Vec<Vec<String>> {
    fn f(mut acc: Vec<Vec<String>>, item: String) -> Vec<Vec<String>> {
        if item == "" {
            acc.push(vec![]);
        } else {
            let n = acc.len() - 1;
            let mut fields = item
                .split(' ')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            acc[n].append(&mut fields);
        }
        acc
    }
    v.into_iter().fold(vec![vec![]], f)
}

fn solve1(passports: &Vec<Vec<String>>) -> i64 {
    fn is_ok(v: &Vec<String>) -> bool {
        let fs = &FIELDS[..];
        for fd in fs.iter() {
            let mut found = false;
            for rec in v.iter() {
                if rec.split(':').take(1).filter(|x| x == fd).count() == 1 {
                    found = true;
                }
            }
            if !found {
                return false;
            }
        }
        return true;
    }

    passports
        .iter()
        .filter(|&x| is_ok(x))
        .count()
        .try_into()
        .unwrap()
}

fn validate(key: &str, val: &str) -> bool {
    match key {
        "byr" => val.parse::<u32>().map_or(false, |x| x >= 1920 && x <= 2002),
        "iyr" => val.parse::<u32>().map_or(false, |x| x >= 2010 && x <= 2020),
        "eyr" => val.parse::<u32>().map_or(false, |x| x >= 2020 && x <= 2030),
        "hgt" => {
            if let Some(s) = val.strip_suffix("in") {
                s.parse::<u8>().map_or(false, |x| x >= 59 && x <= 76)
            } else if let Some(s) = val.strip_suffix("cm") {
                s.parse::<u8>().map_or(false, |x| x >= 150 && x <= 193)
            } else {
                false
            }
        }
        "hcl" => {
            val.len() == 7
                && val.as_bytes()[0] == b'#'
                && val.as_bytes()[1..]
                    .iter()
                    .filter(|&x| (*x >= b'a' && *x <= b'f') || (*x >= b'0' && *x <= b'9'))
                    .count()
                    == 6
        }
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"][..]
                .iter()
                .filter(|&x| *x == val)
                .count()
                == 1
        }
        "pid" => {
            val.chars().count() == 9 && val.chars().filter(|&x| x >= '0' && x <= '9').count() == 9
        }
        "cid" => true,
        _ => false,
    }
}

fn solve2(passports: &Vec<Vec<String>>) -> i64 {
    fn is_ok(v: &Vec<String>) -> bool {
        let fs = &FIELDS[..];
        for fd in fs.iter() {
            let mut found = false;
            let mut valid = false;
            for rec in v.iter() {
                let kvs = rec.split(':').collect::<Vec<&str>>();
                if kvs[0] == *fd {
                    found = true;
                    valid = validate(kvs[0], kvs[1]);
                }
            }
            if !found || !valid {
                return false;
            }
        }
        return true;
    }

    passports
        .iter()
        .filter(|&x| is_ok(x))
        .count()
        .try_into()
        .unwrap()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let passports = mk_passports(v);
    println!("{}", solve1(&passports));
    println!("{}", solve2(&passports));
    Ok(())
}
