// use std::convert::TryInto;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn parse_input(v: &Vec<String>) -> (Vec<(String, Vec<(i64, i64)>)>, Vec<Vec<i64>>) {
    let mut iter = v.iter();
    let rules: Vec<(String, Vec<(i64, i64)>)> = iter
        .by_ref()
        .take_while(|&x| *x != "")
        .map(|s| {
            let ps = s.split(": ").collect::<Vec<&str>>();
            let cs = ps[1]
                .split(" or ")
                .map(|s| {
                    let vals = s
                        .split('-')
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    (vals[0], vals[1])
                })
                .collect::<Vec<(i64, i64)>>();
            (ps[0].to_string(), cs)
        })
        .collect();

    let tickets: Vec<Vec<i64>> = iter
        .filter(|&x| *x != "" && !x.ends_with(":"))
        .map(|s| {
            s.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    (rules, tickets)
}

fn check_validity(rules: &Vec<(String, Vec<(i64, i64)>)>, value: i64) -> bool {
    rules
        .iter()
        .filter(|(_, rs)| {
            rs.iter()
                .filter(|(x, y)| *x <= value && value <= *y)
                .count()
                > 0
        })
        .count()
        > 0
}

fn solve1(rules: &Vec<(String, Vec<(i64, i64)>)>, tickets: &Vec<Vec<i64>>) -> i64 {
    tickets
        .iter()
        .skip(1)
        .flat_map(|xs| xs.iter().filter(|&x| !check_validity(rules, *x)))
        .sum()
}

fn solve2(rules: &Vec<(String, Vec<(i64, i64)>)>, tickets: &Vec<Vec<i64>>) -> i64 {
    let valid_tickets = tickets
        .iter()
        .skip(1)
        .filter(|xs| xs.iter().filter(|&x| check_validity(rules, *x)).count() == xs.len())
        .collect::<Vec<&Vec<i64>>>();

    assert_eq!(valid_tickets[0].len(), rules.len());

    fn is_valid_for_rule(rule: &Vec<(i64, i64)>, val: i64) -> bool {
        rule.iter().filter(|(x, y)| *x <= val && val <= *y).count() > 0
    }

    // h[i] => set of rule possibilities for i-th column in ticket
    let mut h: Vec<HashSet<i64>> = vec![HashSet::new(); rules.len()];
    for c in 0..valid_tickets[0].len() {
        for (i, rule) in rules.iter().enumerate() {
            if valid_tickets
                .iter()
                .map(|x| x[c])
                .filter(|&x| is_valid_for_rule(&rule.1, x))
                .count()
                == valid_tickets.len()
            {
                h[c].insert(i as i64);
            }
        }
    }

    // m[i] => rule number for i-th column.
    let mut m: Vec<i64> = vec![-1; rules.len()];
    let mut mapped = 0;
    while mapped < rules.len() {
        let mut found: i64 = -1;
        for (i, set) in h.iter().enumerate() {
            assert_ne!(set.len(), 0);
            if set.len() == 1 && m[i] == -1 {
                m[i] = *set.iter().nth(0).unwrap();
                found = i as i64;
                break;
            }
        }
        assert_ne!(found, -1);
        mapped += 1;
        for (i, set) in h.iter_mut().enumerate() {
            if i as i64 == found {
                continue;
            }
            set.remove(&m[found as usize]);
        }
    }
    println!("{:?} {}", m, mapped);

    tickets[0]
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let r_idx = m[*i] as usize;
            rules[r_idx].0.starts_with("departure")
        })
        .map(|(_, v)| v)
        .product()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let (rules, tickets) = parse_input(&v);
    println!("{}", solve1(&rules, &tickets));
    println!("{}", solve2(&rules, &tickets));
    Ok(())
}
