// use std::convert::TryInto;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn solve1(v: &Vec<String>) -> i64 {
    let nums = v[0]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut h: HashMap<i64, Vec<i64>> = HashMap::new();

    fn ins(h: &mut HashMap<i64, Vec<i64>>, turn: i64, x: i64) {
        match h.get_mut(&x) {
            Some(v) => {
                if v.len() < 2 {
                    v.push(turn);
                } else {
                    v[0] = v[1];
                    v[1] = turn;
                }
            }
            None => {
                h.insert(x, vec![turn]);
            }
        }
    }

    nums.iter()
        .enumerate()
        .for_each(|(i, x)| ins(&mut h, i as i64 + 1, *x));
    // println!("{:?}", h);
    let mut last = nums[nums.len() - 1];
    for turn in nums.len() + 1..2021 {
        let v = h.get(&last).unwrap();
        let curr = if v.len() == 1 { 0 } else { v[1] - v[0] };
        ins(&mut h, turn as i64, curr);
        // println!("{:?}", h);
        last = curr;
        // println!("{}", last);
    }
    last
}

// Appears to to be slow (35s)
fn solve2(v: &Vec<String>) -> i64 {
    let nums = v[0]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut h: HashMap<i64, Vec<i64>> = HashMap::new();

    fn ins(h: &mut HashMap<i64, Vec<i64>>, turn: i64, x: i64) {
        match h.get_mut(&x) {
            Some(v) => {
                if v.len() < 2 {
                    v.push(turn);
                } else {
                    v[0] = v[1];
                    v[1] = turn;
                }
            }
            None => {
                h.insert(x, vec![turn]);
            }
        }
    }

    nums.iter()
        .enumerate()
        .for_each(|(i, x)| ins(&mut h, i as i64 + 1, *x));
    // println!("{:?}", h);
    let mut last = nums[nums.len() - 1];
    for turn in nums.len() + 1..30_000_000 + 1 {
        let v = h.get(&last).unwrap();
        let curr = if v.len() == 1 { 0 } else { v[1] - v[0] };
        ins(&mut h, turn as i64, curr);
        // println!("{:?}", h);
        last = curr;
        // println!("{}", last);
    }
    last
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
