// use std::convert::TryInto;
use std::io::{self, BufRead};

fn solve1(v: &Vec<String>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    v.iter().for_each(|s| {
        let (act, val) = s.split_at(1);
        let n = val.parse::<i64>().unwrap();
        match act {
            "N" => y += n,
            "S" => y -= n,
            "E" => x += n,
            "W" => x -= n,
            "L" => {
                d = d + n;
                if d >= 360 {
                    d = d % 360;
                }
            }
            "R" => {
                d = d - n;
                if d < 0 {
                    d += 360;
                }
            }
            "F" => match d {
                0 => x += n,
                90 => y += n,
                180 => x -= n,
                270 => y -= n,
                _ => {}
            },
            _ => {}
        }
    });
    if x < 0 {
        x = -x;
    }
    if y < 0 {
        y = -y;
    }
    x + y
}

fn solve2(v: &Vec<String>) -> i64 {
    fn rotate_left(dx: i64, dy: i64, mut angle: i64) -> (i64, i64) {
        while angle < 0 {
            angle += 360;
        }
        if angle >= 360 {
            angle = angle % 360;
        }
        match angle {
            0 => (dx, dy),
            90 => (-dy, dx),
            180 => (-dx, -dy),
            270 => (dy, -dx),
            _ => unreachable!(),
        }
    }
    let (mut sx, mut sy, mut wx, mut wy) = (0, 0, 10, 1);
    v.iter().for_each(|s| {
        let (act, val) = s.split_at(1);
        let n = val.parse::<i64>().unwrap();
        match act {
            "N" => wy += n,
            "S" => wy -= n,
            "E" => wx += n,
            "W" => wx -= n,
            "L" => {
                let (ox, oy) = rotate_left(wx, wy, n);
                wx = ox;
                wy = oy;
            }
            "R" => {
                let (ox, oy) = rotate_left(wx, wy, 360 - n);
                wx = ox;
                wy = oy;
            }
            "F" => {
                sx += n * wx;
                sy += n * wy;
            }
            _ => {}
        }
    });
    if sx < 0 {
        sx = -sx;
    }
    if sy < 0 {
        sy = -sy;
    }
    sx + sy
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
