use std::collections::HashSet;
use std::io::{self, BufRead};

fn seat_id(s: &String) -> u64 {
    let (r, _, c, _) = s
        .chars()
        .enumerate()
        .fold((0, 127, 0, 7), |(rl, rh, cl, ch), (i, c)| {
            if i < 7 {
                let m = (rl + rh + 1) / 2;
                if c == 'F' {
                    (rl, m - 1, cl, ch)
                } else {
                    (m, rh, cl, ch)
                }
            } else {
                let m = (cl + ch + 1) / 2;
                if c == 'L' {
                    (rl, rh, cl, m - 1)
                } else {
                    (rl, rh, m, ch)
                }
            }
        });

    r * 8 + c
}

fn solve1(v: &Vec<String>) -> u64 {
    v.iter().map(seat_id).max().unwrap()
}

fn solve2(v: &Vec<String>) -> u64 {
    let s = v.iter().map(seat_id).collect::<HashSet<u64>>();
    for i in 0..(128 * 8 - 1) {
        if !s.contains(&i) {
            if s.contains(&(i + 1)) && s.contains(&(i - 1)) {
                return i;
            }
        }
    }
    return 100000000;
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
