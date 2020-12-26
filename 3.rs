use std::io::{self, BufRead};

fn process_map(v: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let r = v.len();
    let c = v[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut res = 0;
    while x < r {
        if v[x][y % c] == '#' {
            res = res + 1;
        }
        x = x + dx;
        y = y + dy;
    }
    res
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<Vec<char>> = handle
        .lines()
        .map(|r| r.map(|x| x.chars().collect::<Vec<char>>()))
        .collect::<Result<_, io::Error>>()?;
    println!("{}", process_map(&v, 1, 3));
    let slopes = &[(1, 3), (1, 1), (1, 5), (1, 7), (2, 1)][..];
    let mut p = 1;
    for slope in slopes {
        p = p * process_map(&v, slope.0, slope.1);
    }
    println!("{}", p);
    Ok(())
}
