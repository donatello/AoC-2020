use std::collections::HashSet;
use std::io::{self, BufRead, Error, ErrorKind};

fn doit(v: &Vec<i64>) -> i64 {
    let mut m = HashSet::new();
    let mut res = -1;
    v.iter().for_each(|i| {
        if m.contains(&(2020 - i)) {
            res = i * (2020 - i);
        };
        m.insert(i);
    });
    res
}

fn doit2(v: &Vec<i64>) -> i64 {
    let m: HashSet<i64> = v.iter().cloned().collect();
    for (i, a) in v.iter().enumerate() {
        for (j, b) in v.iter().enumerate() {
            if i == j {
                continue;
            }
            let n = 2020 - a - b;
            if m.contains(&n) {
                return a * b * n;
            }
        }
    }
    return -1;
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<i64> = handle
        .lines()
        .map(|line| line.and_then(|s| s.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect::<Result<Vec<i64>, io::Error>>()?;
    println!("{}", doit(&v));
    println!("{}", doit2(&v));
    Ok(())
}
