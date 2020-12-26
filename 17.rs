// use std::convert::TryInto;
// use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve1(v: &Vec<String>) -> i64 {
    let mut curr = vec![vec![vec![false; v[0].len() + 14]; v.len() + 14]; 15];
    let mut prev = vec![vec![vec![false; v[0].len() + 14]; v.len() + 14]; 15];
    for (i, l) in v.iter().enumerate() {
        for (j, b) in l.bytes().enumerate() {
            if b == b'#' {
                curr[7][i + 7][j + 7] = true;
            }
        }
    }

    let dx = &[-1, 0, 1][..];
    let nbrs = dx
        .iter()
        .flat_map(|x| {
            dx.iter()
                .flat_map(move |y| dx.iter().map(move |z| (x.clone(), y.clone(), z.clone())))
        })
        .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
        .collect::<Vec<(i64, i64, i64)>>();
    for _cycle in 0..6 {
        {
            let t = curr;
            curr = prev;
            prev = t;
        }
        for i in 0..curr.len() {
            for j in 0..curr[0].len() {
                for k in 0..curr[0][0].len() {
                    curr[i][j][k] = prev[i][j][k];
                    let active_nbrs = nbrs
                        .iter()
                        .filter_map(|(di, dj, dk)| {
                            let ii = i as i64 + di;
                            let jj = j as i64 + dj;
                            let kk = k as i64 + dk;
                            if 0 <= ii
                                && ii < curr.len() as i64
                                && 0 <= jj
                                && jj < curr[0].len() as i64
                                && 0 <= kk
                                && kk < curr[0][0].len() as i64
                            {
                                Some((ii as usize, jj as usize, kk as usize))
                            } else {
                                None
                            }
                        })
                        .filter(|(x, y, z)| prev[*x][*y][*z])
                        .count();
                    if prev[i][j][k] {
                        if !(active_nbrs == 2 || active_nbrs == 3) {
                            curr[i][j][k] = false;
                        }
                    } else {
                        if active_nbrs == 3 {
                            curr[i][j][k] = true;
                        }
                    }
                }
            }
        }
    }
    curr.iter()
        .flat_map(|x| x.iter().flat_map(|y| y.iter().filter(|&x| *x)))
        .count() as i64
}

fn solve2(v: &Vec<String>) -> i64 {
    let mut curr = vec![vec![vec![vec![false; v[0].len() + 14]; v.len() + 14]; 15]; 15];
    let mut prev = vec![vec![vec![vec![false; v[0].len() + 14]; v.len() + 14]; 15]; 15];
    for (i, l) in v.iter().enumerate() {
        for (j, b) in l.bytes().enumerate() {
            if b == b'#' {
                curr[7][7][i + 7][j + 7] = true;
            }
        }
    }

    let dx = &[-1, 0, 1][..];
    let nbrs = dx
        .iter()
        .flat_map(|x| {
            dx.iter().flat_map(move |y| {
                dx.iter().flat_map(move |z| {
                    dx.iter()
                        .map(move |w| (x.clone(), y.clone(), z.clone(), w.clone()))
                })
            })
        })
        .filter(|(x, y, z, w)| *x != 0 || *y != 0 || *z != 0 || *w != 0)
        .collect::<Vec<(i64, i64, i64, i64)>>();
    for _cycle in 0..6 {
        {
            let t = curr;
            curr = prev;
            prev = t;
        }
        for i in 0..curr.len() {
            for j in 0..curr[0].len() {
                for k in 0..curr[0][0].len() {
                    for l in 0..curr[0][0][0].len() {
                        curr[i][j][k][l] = prev[i][j][k][l];
                        let active_nbrs = nbrs
                            .iter()
                            .filter_map(|(di, dj, dk, dl)| {
                                let ii = i as i64 + di;
                                let jj = j as i64 + dj;
                                let kk = k as i64 + dk;
                                let ll = l as i64 + dl;
                                if 0 <= ii
                                    && ii < curr.len() as i64
                                    && 0 <= jj
                                    && jj < curr[0].len() as i64
                                    && 0 <= kk
                                    && kk < curr[0][0].len() as i64
                                    && 0 <= ll
                                    && ll < curr[0][0][0].len() as i64
                                {
                                    Some((ii as usize, jj as usize, kk as usize, ll as usize))
                                } else {
                                    None
                                }
                            })
                            .filter(|(x, y, z, w)| prev[*x][*y][*z][*w])
                            .count();
                        if prev[i][j][k][l] {
                            if !(active_nbrs == 2 || active_nbrs == 3) {
                                curr[i][j][k][l] = false;
                            }
                        } else {
                            if active_nbrs == 3 {
                                curr[i][j][k][l] = true;
                            }
                        }
                    }
                }
            }
        }
    }
    curr.iter()
        .flat_map(|x| {
            x.iter()
                .flat_map(|y| y.iter().flat_map(|z| z.iter().filter(|&x| *x)))
        })
        .count() as i64
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
