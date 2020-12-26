use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};
//use std::iter;

fn parse_input(v: &Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let mut iter = v.iter();
    let p1 = iter
        .by_ref()
        .skip(1)
        .take_while(|x| *x != "")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let p2 = iter
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    (p1, p2)
}

fn solve1(p1: &Vec<u64>, p2: &Vec<u64>) -> u64 {
    let mut d1 = p1.iter().cloned().collect::<VecDeque<u64>>();
    let mut d2 = p2.iter().cloned().collect::<VecDeque<u64>>();
    while d1.len() > 0 && d2.len() > 0 {
        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();
        if c1 > c2 {
            d1.push_back(c1);
            d1.push_back(c2);
        } else {
            d2.push_back(c2);
            d2.push_back(c1);
        }
    }
    let winner = if d1.len() == 0 { d2 } else { d1 };
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * v)
        .sum()
}

fn solve2(p1: &Vec<u64>, p2: &Vec<u64>) -> u64 {
    // returns true if player 1 wins, false otherwise.
    fn rec_combat(d1: &mut VecDeque<u8>, d2: &mut VecDeque<u8>) -> bool {
        let mut h: HashSet<(VecDeque<u8>, VecDeque<u8>)> = HashSet::new();
        while d1.len() > 0 && d2.len() > 0 {
            if h.contains(&(d1.clone(), d2.clone())) {
                return true;
            }
            h.insert((d1.clone(), d2.clone()));
            let c1 = d1.pop_front().unwrap();
            let c2 = d2.pop_front().unwrap();
            if d1.len() >= c1 as usize && d2.len() >= c2 as usize {
                // recurse
                let mut nd1 = d1
                    .iter()
                    .take(c1 as usize)
                    .cloned()
                    .collect::<VecDeque<u8>>();
                let mut nd2 = d2
                    .iter()
                    .take(c2 as usize)
                    .cloned()
                    .collect::<VecDeque<u8>>();
                let res = rec_combat(&mut nd1, &mut nd2);
                if res {
                    d1.push_back(c1);
                    d1.push_back(c2);
                } else {
                    d2.push_back(c2);
                    d2.push_back(c1);
                }
            } else {
                if c1 > c2 {
                    d1.push_back(c1);
                    d1.push_back(c2);
                } else {
                    d2.push_back(c2);
                    d2.push_back(c1);
                }
            }
        }
        d2.len() == 0
    }

    let mut d1 = p1
        .iter()
        .cloned()
        .map(|x| x as u8)
        .collect::<VecDeque<u8>>();
    let mut d2 = p2
        .iter()
        .cloned()
        .map(|x| x as u8)
        .collect::<VecDeque<u8>>();
    rec_combat(&mut d1, &mut d2);
    println!("Player {} won", if d1.len() == 0 { 2 } else { 1 });
    let winner = if d1.len() == 0 { d2 } else { d1 };
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * (*v) as u64)
        .sum()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let (p1, p2) = parse_input(&v);
    println!("{}", solve1(&p1, &p2));
    println!("{}", solve2(&p1, &p2));
    Ok(())
}
