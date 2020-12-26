use std::collections::HashSet;
// use std::collections::VecDeque;
use std::io::{self, BufRead};
//use std::iter;

fn initial_black_tiles(v: &[String]) -> HashSet<(i64, i64)> {
    let mut h = HashSet::new();
    // e, se, sw, w, nw, ne
    v.iter().map(|x| x.as_str()).for_each(|mut s| {
        let mut pos: (i64, i64) = (0, 0);
        loop {
            if s.starts_with("e") {
                s = &s[1..];
                pos = (pos.0, pos.1 + 2);
            } else if s.starts_with("se") {
                s = &s[2..];
                pos = (pos.0 - 1, pos.1 + 1);
            } else if s.starts_with("sw") {
                s = &s[2..];
                pos = (pos.0 - 1, pos.1 - 1);
            } else if s.starts_with("w") {
                s = &s[1..];
                pos = (pos.0, pos.1 - 2);
            } else if s.starts_with("nw") {
                s = &s[2..];
                pos = (pos.0 + 1, pos.1 - 1);
            } else if s.starts_with("ne") {
                s = &s[2..];
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                break;
            }
        }
        if h.contains(&pos) {
            h.remove(&pos);
        } else {
            h.insert(pos.clone());
        }
    });
    h
}

fn solve1(v: &[String]) -> u64 {
    let h = initial_black_tiles(v);
    h.len() as u64
}

fn solve2(v: &[String]) -> u64 {
    let mut h = initial_black_tiles(v);
    let nbrs: Vec<(i64, i64)> = [(0, 2), (-1, 1), (-1, -1), (0, -2), (1, -1), (1, 1)]
        .iter()
        .cloned()
        .collect::<Vec<(i64, i64)>>();
    for i in 0..100 {
        println!("iter: {}", i);
        let black_flips = h
            .iter()
            .cloned()
            .filter_map(|(x, y)| {
                let black_nbrs = nbrs
                    .iter()
                    .cloned()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|nbr_pos| h.contains(nbr_pos))
                    .count();
                if black_nbrs == 0 || black_nbrs > 2 {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect::<Vec<(i64, i64)>>();

        let mut white_candidates = HashSet::new();
        for (x1, y1) in h.iter().cloned() {
            for (x2, y2) in h.iter().cloned() {
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                let dx = (x1 - x2).abs();
                let dy = (y1 - y2).abs();
                if dx + dy <= 4 {
                    // Add all white tile nbrs of the two black tiles.
                    let w1_nbrs = nbrs
                        .iter()
                        .cloned()
                        .map(|(dx, dy)| (x1 + dx, y1 + dy))
                        .filter(|np| !h.contains(np))
                        .collect::<HashSet<(i64, i64)>>();
                    let w2_nbrs = nbrs
                        .iter()
                        .cloned()
                        .map(|(dx, dy)| (x2 + dx, y2 + dy))
                        .filter(|np| !h.contains(np))
                        .collect::<HashSet<(i64, i64)>>();
                    w1_nbrs.intersection(&w2_nbrs).for_each(|p| {
                        white_candidates.insert(p.clone());
                    });
                }
            }
        }
        println!("wcands len: {}", white_candidates.len());

        let white_flips = white_candidates
            .iter()
            .cloned()
            .filter_map(|(x, y)| {
                let black_nbrs = nbrs
                    .iter()
                    .cloned()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|nbr_pos| h.contains(nbr_pos))
                    .count();
                if black_nbrs == 2 {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect::<Vec<(i64, i64)>>();

        println!(
            "white tiles added: {} black tiles removed: {}",
            white_flips.len(),
            black_flips.len()
        );

        // Finally update h
        black_flips.iter().cloned().for_each(|p| {
            h.remove(&p);
        });
        white_flips.iter().cloned().for_each(|p| {
            h.insert(p);
        });
    }
    h.len() as u64
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
