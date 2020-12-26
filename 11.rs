// use std::convert::TryInto;
use std::io::{self, BufRead};

fn solve1(v: &Vec<Vec<u8>>) -> i64 {
    let ds = (-1..2)
        .into_iter()
        .flat_map(|x| (-1..2).into_iter().map(move |y| (x, y)))
        .filter(|(x, y)| *x != 0 || *y != 0)
        .collect::<Vec<(i64, i64)>>();
    let r = v.len();
    let c = v[0].len();
    let mut prev = v.clone();
    let mut t = vec![vec![b'.'; c]; r];
    loop {
        for i in 0..r {
            for j in 0..c {
                t[i][j] = prev[i][j];
                let occ_nbrs = ds
                    .iter()
                    .map(|(di, dj)| ((i as i64) + di, (j as i64) + dj))
                    .filter_map(|(x, y)| {
                        if 0 <= x && x < r as i64 && 0 <= y && y < c as i64 {
                            Some((x as usize, y as usize))
                        } else {
                            None
                        }
                    })
                    .filter(|(x, y)| prev[*x][*y] == b'#')
                    .count();
                match prev[i][j] {
                    b'L' => {
                        if occ_nbrs == 0 {
                            t[i][j] = b'#';
                        }
                    }
                    b'#' => {
                        if occ_nbrs >= 4 {
                            t[i][j] = b'L';
                        }
                    }
                    _ => {}
                }
            }
        }
        if prev == t {
            break;
        }
        let k = t;
        t = prev;
        prev = k;
    }
    prev.iter()
        .flat_map(|x| x.iter().filter(|&y| *y == b'#'))
        .count() as i64
}

fn solve2(v: &Vec<Vec<u8>>) -> i64 {
    let ds = (-1..2)
        .into_iter()
        .flat_map(|x| (-1..2).into_iter().map(move |y| (x, y)))
        .filter(|(x, y)| *x != 0 || *y != 0)
        .collect::<Vec<(i64, i64)>>();
    let r = v.len();
    let c = v[0].len();
    let mut nbrs: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; c]; r];
    for i in 0..r {
        for j in 0..c {
            ds.iter().for_each(|(di, dj)| {
                for k in 1.. {
                    let x = (i as i64) + k * di;
                    let y = (j as i64) + k * dj;
                    if 0 <= x && x < r as i64 && 0 <= y && y < c as i64 {
                        let p = x as usize;
                        let q = y as usize;
                        if v[p][q] == b'L' || v[p][q] == b'#' {
                            nbrs[i][j].push((p, q));
                            break;
                        }
                    } else {
                        break;
                    }
                }
            })
        }
    }

    let mut prev = v.clone();
    let mut t = vec![vec![b'.'; c]; r];
    loop {
        for i in 0..r {
            for j in 0..c {
                t[i][j] = prev[i][j];
                let occ_nbrs = nbrs[i][j]
                    .iter()
                    .filter(|(p, q)| prev[*p][*q] == b'#')
                    .count();
                match prev[i][j] {
                    b'L' => {
                        if occ_nbrs == 0 {
                            t[i][j] = b'#';
                        }
                    }
                    b'#' => {
                        if occ_nbrs >= 5 {
                            t[i][j] = b'L';
                        }
                    }
                    _ => {}
                }
            }
        }
        if prev == t {
            break;
        }
        let k = t;
        t = prev;
        prev = k;
    }
    prev.iter()
        .flat_map(|x| x.iter().filter(|&y| *y == b'#'))
        .count() as i64
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let area = v
        .iter()
        .map(|x| x.bytes().collect())
        .collect::<Vec<Vec<u8>>>();
    println!("{}", solve1(&area));
    println!("{}", solve2(&area));
    Ok(())
}
