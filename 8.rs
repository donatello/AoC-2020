// use std::convert::TryInto;
use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug)]
enum I {
    Jmp(i64),
    Nop(i64),
    Acc(i64),
}

fn parse_ins(s: &str) -> I {
    let ps = s.splitn(2, ' ').collect::<Vec<&str>>();
    match ps[0] {
        "nop" => I::Nop(ps[1].parse::<i64>().unwrap()),
        "acc" => I::Acc(ps[1].parse::<i64>().unwrap()),
        "jmp" => I::Jmp(ps[1].parse::<i64>().unwrap()),
        _ => unreachable!(),
    }
}

fn solve1(ins: &Vec<I>) -> i64 {
    let mut acc = 0;
    let mut s: HashSet<i64> = HashSet::new();
    let mut ip: i64 = 0;
    while !s.contains(&ip) {
        s.insert(ip);
        match ins[ip as usize] {
            I::Jmp(n) => ip = ip + n,
            I::Nop(_) => ip = ip + 1,
            I::Acc(n) => {
                acc = acc + n;
                ip = ip + 1;
            }
        }
    }
    return acc;
}

fn solve2(ins: &Vec<I>) -> i64 {
    let mut ch = 0;
    loop {
        match ins[ch] {
            I::Acc(_) => {
                ch += 1;
                continue;
            }
            _ => {}
        }
        let mut acc = 0;
        let mut s: HashSet<i64> = HashSet::new();
        let mut ip: i64 = 0;
        while !s.contains(&ip) && (ip as usize) < ins.len() {
            s.insert(ip);
            let mut i = &ins[ip as usize];
            let t;
            if ch == ip as usize {
                t = match ins[ch] {
                    I::Nop(n) => I::Jmp(n),
                    I::Jmp(n) => I::Nop(n),
                    _ => unreachable!(),
                };
                i = &t;
            }
            match i {
                I::Jmp(n) => ip = ip + n,
                I::Nop(_) => ip = ip + 1,
                I::Acc(n) => {
                    acc = acc + n;
                    ip = ip + 1;
                }
            }
        }
        if (ip as usize) >= ins.len() {
            return acc;
        } else {
            ch += 1;
        }
    }
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let ins: Vec<I> = v.iter().map(|x| parse_ins(&x)).collect();
    println!("{}", solve1(&ins));
    println!("{}", solve2(&ins));
    Ok(())
}
