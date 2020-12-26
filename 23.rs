use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufRead};
//use std::iter;

fn rotate_left(v: &mut VecDeque<u8>) {
    let t = v.pop_front().unwrap();
    v.push_back(t);
}

fn solve1(s: &str) -> String {
    let mut cups = s
        .chars()
        .map(|x| (x as u32 - '0' as u32) as u8)
        .collect::<VecDeque<u8>>();
    let max_label = cups.iter().cloned().max().unwrap();
    for _ in 0..100 {
        let current_cup = cups.get(0).unwrap().clone();
        rotate_left(&mut cups);
        let picked_up = cups.iter().take(3).cloned().collect::<Vec<u8>>();
        cups = cups.into_iter().skip(3).collect::<VecDeque<u8>>();
        let mut dest = current_cup - 1;
        loop {
            if dest < 1 {
                dest = max_label;
            }
            if picked_up.iter().filter(|&x| *x == dest).count() > 0 {
                dest -= 1;
            } else {
                break;
            }
        }
        while cups.get(cups.len() - 1).unwrap() != &dest {
            rotate_left(&mut cups);
        }
        picked_up.iter().rev().for_each(|x| cups.push_front(*x));
        while cups.get(cups.len() - 1).unwrap() != &current_cup {
            rotate_left(&mut cups);
        }
    }

    while cups.get(0).unwrap() != &1 {
        rotate_left(&mut cups);
    }

    cups.pop_front();

    cups.into_iter()
        .map(|x| (x + '0' as u8) as char)
        .collect::<String>()
}

fn solve2(s: &str) -> u64 {
    let cups = s
        .chars()
        .map(|x| (x as u32 - '0' as u32) as usize)
        .chain((10..1_000_001).into_iter())
        .collect::<Vec<usize>>();
    let cup_pos_map = cups
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, n)| (n, i as usize))
        .collect::<HashMap<usize, usize>>();

    // next represents the next pointers in the circularly linked list.
    let mut next = (1..1_000_000).into_iter().collect::<Vec<usize>>();
    next.push(0);

    let mut curr_index = 0;
    for i in 1..10_000_000 {
        if i % 1_000_000 == 0 {
            println!("i: {}", i);
        }
        let c1 = next[curr_index];
        let c2 = next[c1];
        let c3 = next[c2];
        next[curr_index] = next[c3];
        let mut dest = cups[curr_index] - 1;
        loop {
            if dest < 1 {
                dest = 1_000_000;
            }
            if dest == cups[c1] || dest == cups[c2] || dest == cups[c3] {
                dest -= 1;
            } else {
                break;
            }
        }
        let dest_index = *cup_pos_map.get(&dest).unwrap();
        next[c3] = next[dest_index];
        next[dest_index] = c1;
        curr_index = next[curr_index];
    }

    let cup_1 = *cup_pos_map.get(&1).unwrap();
    (cups[next[cup_1]] as u64) * (cups[next[next[cup_1]]] as u64)
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v[0]));
    println!("{}", solve2(&v[0]));
    Ok(())
}
