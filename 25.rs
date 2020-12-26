//use std::collections::HashSet;
// use std::collections::VecDeque;
use std::io::{self, BufRead};
//use std::iter;

const MOD: u64 = 20201227;

fn solve1(v: &[String]) -> u64 {
    let p1 = v[0].parse::<u64>().unwrap();
    let p2 = v[1].parse::<u64>().unwrap();

    let mut s = 1;
    let mut e1 = 0;
    let mut e2 = 0;
    for i in 1.. {
        if i % 1000000 == 0 {
            println!("i: {}, e1: {}, e2: {}", i, e1, e2)
        }
        s = (s * 7) % MOD;
        if s == p1 {
            e1 = i;
            println!("i: {}, e1: {}, e2: {}", i, e1, e2)
        }
        if s == p2 {
            e2 = i;
            println!("i: {}, e1: {}, e2: {}", i, e1, e2)
        }
        if e1 != 0 && e2 != 0 {
            break;
        }
    }

    fn tx(subject: u64, loop_size: u64) -> u64 {
        if loop_size == 1 {
            return subject;
        }
        let d = loop_size / 2;
        let v = tx(subject, d);
        let r = (v * v) % MOD;

        if loop_size % 2 == 1 {
            (r * subject) % MOD
        } else {
            r
        }
    }

    // calculate either: p2^e1 or p1^e2 mod p
    let enc1 = tx(p2, e1);
    let enc2 = tx(p1, e2);
    assert_eq!(enc1, enc2);
    enc1
}

fn solve2(v: &[String]) -> u64 {
    0
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
