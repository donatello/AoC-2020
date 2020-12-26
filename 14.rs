// use std::convert::TryInto;
use std::collections::HashMap;
use std::io::{self, BufRead};

enum I {
    Mask(String),
    Mem(i64, i64),
}

fn parse_input(s: String) -> I {
    let ps = s.split(" = ").collect::<Vec<&str>>();
    if ps[0] == "mask" {
        I::Mask(ps[1].to_string())
    } else {
        let mem = ps[0]
            .trim_matches(&['m', 'e', 'm', '[', ']'][..])
            .parse::<i64>()
            .unwrap();
        let val = ps[1].parse::<i64>().unwrap();
        I::Mem(mem, val)
    }
}

fn solve1(v: &Vec<I>) -> i64 {
    fn apply_mask(mask: &str, val: i64) -> i64 {
        let mask_bytes = mask.as_bytes();
        let mut res: i64 = val;
        for i in 0..36 {
            let bit_index = 35 - i;
            let k = 1 << bit_index;
            match mask_bytes[i] {
                b'X' => continue,
                b'1' => res = res | k,
                b'0' => {
                    if res & k > 0 {
                        res = res ^ k;
                    }
                }
                _ => {}
            }
        }
        res
    }

    let mut h: HashMap<i64, i64> = HashMap::new();
    let mut mask: String = "".to_string();
    v.iter().for_each(|ins| match ins {
        I::Mask(s) => mask = s.to_string(),
        I::Mem(addr, val) => {
            h.insert(*addr, apply_mask(&mask, *val));
        }
    });
    h.iter().map(|(_, &v)| v).sum()
}

fn solve2(v: &Vec<I>) -> i64 {
    fn apply_mask(mask: &str, val: i64) -> Vec<i64> {
        let mask_bytes = mask.as_bytes();
        let mut res = val;
        for i in 0..36 {
            let bit_index = 35 - i;
            let k = 1 << bit_index;
            match mask_bytes[i] {
                b'1' => res = res | k,
                _ => {}
            }
        }

        let x_count = mask.bytes().filter(|&x| x == b'X').count();
        if x_count == 0 {
            return vec![res];
        }

        let xids = mask
            .bytes()
            .enumerate()
            .filter(|(_, x)| *x == b'X')
            .map(|(i, _)| 35 - i as i64)
            .collect::<Vec<i64>>();

        (0..(1 << x_count))
            .into_iter()
            .map(|x| {
                let mut v = res;
                xids.iter().enumerate().for_each(|(i, ix)| {
                    let ith_x = x & (1 << i);
                    let k = 1 << ix;
                    if ith_x > 0 {
                        // set ix-th bit in v
                        v = v | k
                    } else {
                        // reset ix-th bit in v
                        if v & k > 0 {
                            v = v ^ (1 << ix);
                        }
                    }
                });
                v
            })
            .collect::<Vec<i64>>()
    }

    let mut h: HashMap<i64, i64> = HashMap::new();
    let mut mask: String = "".to_string();
    v.iter().for_each(|ins| match ins {
        I::Mask(s) => mask = s.to_string(),
        I::Mem(addr, val) => apply_mask(&mask, *addr).iter().for_each(|x| {
            h.insert(*x, *val);
        }),
    });
    h.iter().map(|(_, &v)| v).sum()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let ins = v.iter().map(|x| parse_input(x.clone())).collect::<Vec<I>>();
    println!("{}", solve1(&ins));
    println!("{}", solve2(&ins));
    Ok(())
}
