// use std::convert::TryInto;
use std::io::{self, BufRead};

fn solve1(ns: &mut Vec<i64>) -> i64 {
    ns.sort_unstable();
    let mut ones = 0;
    let mut threes = 1;
    let mut prev = 0;
    ns.iter().for_each(|&x| {
        if x - prev == 1 {
            ones += 1;
        } else if x - prev == 3 {
            threes += 1;
        }
        prev = x;
    });
    ones * threes
}

fn solve2(ns: &Vec<i64>) -> i64 {
    let mut c = vec![0; ns.len()];
    for i in 0..ns.len() {
        if i > 0 && ns[i] - ns[i - 1] <= 3 {
            c[i] += c[i - 1];
        }
        if i > 1 && ns[i] - ns[i - 2] <= 3 {
            c[i] += c[i - 2];
        }
        if i > 2 && ns[i] - ns[i - 3] <= 3 {
            c[i] += c[i - 3];
        }
        if ns[i] <= 3 {
            c[i] += 1;
        }
    }
    c[ns.len() - 1]
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let mut ins: Vec<i64> = v.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    println!("{}", solve1(&mut ins));
    println!("{}", solve2(&ins));
    Ok(())
}
