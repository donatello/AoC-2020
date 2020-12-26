// use std::convert::TryInto;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve1(ns: &Vec<i64>) -> i64 {
    let mut s = ns[0..25].iter().cloned().collect::<HashSet<i64>>();
    for i in 25..ns.len() {
        if s.iter()
            .filter(|&x| s.contains(&(ns[i] - *x)) && 2 * (*x) != ns[i])
            .count()
            == 0
        {
            return ns[i];
        }
        s.remove(&ns[i - 25]);
        s.insert(ns[i]);
    }
    -1
}

fn solve2(ns: &Vec<i64>) -> i64 {
    let n = solve1(ns);
    let s = ns[1..].iter().fold(vec![ns[0]], |mut v, x| {
        v.push(x + v[v.len() - 1]);
        v
    });
    for i in 0..ns.len() {
        for j in i + 1..ns.len() {
            if s[j] - s[i] == n {
                let (low, hi) = (
                    ns[i + 1..j + 1].iter().min().unwrap(),
                    ns[i + 1..j + 1].iter().max().unwrap(),
                );
                return low + hi;
            }
        }
    }
    0
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let ins: Vec<i64> = v.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    println!("{}", solve1(&ins));
    println!("{}", solve2(&ins));
    Ok(())
}
