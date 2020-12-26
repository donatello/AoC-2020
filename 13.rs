// use std::convert::TryInto;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve1(v: &Vec<String>) -> i64 {
    let t = v[0].parse::<i64>().unwrap();
    let ids: Vec<i64> = v[1]
        .split(&[',', 'x'][..])
        .filter(|&x| x != "")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let (min_id, min_wait) = ids
        .iter()
        .map(|x| {
            let mut wait = t % x;
            if wait != 0 {
                wait = x - wait;
            }
            (x, wait)
        })
        .min_by_key(|(_, y)| y.clone())
        .unwrap();
    min_id * min_wait
}

fn solve2(v: &Vec<String>) -> i64 {
    let ns: Vec<(i64, i64)> = v[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| match s.parse::<i64>() {
            Ok(n) => {
                if i == 0 {
                    Some((n, 0))
                } else {
                    Some((n, n - (i as i64)))
                }
            }
            Err(_) => None,
        })
        .collect();

    // t must have the form ns[i].0 * x[i] + ns[i].1, for some integer x[i] for
    // all i, s.t. 0 <= i < ns.len().

    // Initially, t is of the form ns[0].0 * x[0] + ns[0].1.

    let mut a = ns[0].0;
    let mut b = ns[0].1; // known to be 0.

    fn f(a: i64, b: i64, p: i64, q: i64) -> (i64, i64) {
        // t is of the form a*x+b. t must also be of the form p*y+q. Calculate
        // (r, s), such that t is of the form r*z+s, combining the previous
        // constraints.

        //     a*x+b == p*y+q
        // ==> a*x+b-q == p*y
        // ==> a*x+b-q ==p== 0 (notation for modular congruence mod p).
        // ==> a*x     ==p== q-b

        let mut r1 = (q - b) % p;
        if r1 < 0 {
            r1 += p;
        }

        // ==> a*x ==p== r1, where 0 <= r1
        // x is some integer, a is known.
        let mut set: HashSet<i64> = HashSet::new();
        let mut rems: Vec<i64> = vec![];
        let a_rem = a % p;
        for x in 0.. {
            let v = (a_rem * x) % p;
            if set.contains(&v) {
                if v != 0 {
                    // NOT SURE IF THIS IS IMPOSSIBLE!
                    panic!("Unexpected non-zero repeater!");
                }
                break;
            }
            rems.push(v);
            set.insert(v);
        }
        let offset = rems.iter().enumerate().find(|(_, &v)| v == r1).unwrap().0 as i64;
        let period = rems.len() as i64;
        // x is of the form period*y + offset
        // a*x + b == a*(period*y + offset) + b
        //         == a*period*y + a*offset + b
        (a * period, a * offset + b)
    }

    ns.iter().for_each(|(p, q)| {
        let (r, s) = f(a, b, *p, *q);
        a = r;
        b = s;
    });
    b
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
