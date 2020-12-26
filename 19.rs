// use std::convert::TryInto;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;

#[derive(Debug)]
enum R {
    P(Vec<Vec<usize>>), // production
    T(char),            // terminal
}

fn parse_input(v: &[String]) -> (HashMap<usize, R>, &[String]) {
    let (rules, strings) =
        if let Some((i, _)) = v.iter().enumerate().filter(|(_, x)| *x == "").nth(0) {
            (&v[..i], &v[i + 1..])
        } else {
            unreachable!();
        };
    let rs = rules
        .iter()
        .map(|s| {
            let ps = s.split(": ").collect::<Vec<&str>>();
            assert_eq!(ps.len(), 2);
            let rid: usize = ps[0].parse().unwrap();
            let rs = if ps[1].contains('"') {
                let c = ps[1].chars().nth(1).unwrap();
                R::T(c)
            } else {
                R::P(
                    ps[1]
                        .split(" | ")
                        .map(|x| {
                            x.split(' ')
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect::<Vec<usize>>()
                        })
                        .collect::<Vec<Vec<usize>>>(),
                )
            };
            (rid, rs)
        })
        .collect::<HashMap<usize, R>>();
    (rs, strings)
}

fn solve1(rules: &HashMap<usize, R>, ss: &[String]) -> usize {
    fn match_rules(rules: &HashMap<usize, R>, s: &[char], stack: &mut Vec<usize>) -> bool {
        if stack.len() == 0 {
            return s.len() == 0;
        }
        if s.len() == 0 {
            return false;
        }
        let rid = stack.pop().unwrap();
        let r = rules.get(&rid).unwrap();
        match r {
            R::T(c) => {
                if *c == s[0] {
                    return match_rules(rules, &s[1..], stack);
                }
            }
            R::P(subrules) => {
                let saved_stack = stack.clone();
                for subrule in subrules.iter() {
                    subrule.iter().rev().for_each(|i| stack.push(*i));
                    if match_rules(rules, s, stack) {
                        return true;
                    }
                    *stack = saved_stack.clone();
                }
            }
        }
        false
    }
    ss.iter()
        .filter(|x| match_rules(rules, &x.chars().collect::<Vec<char>>(), &mut vec![0]))
        .count()
}

fn solve2(rules: &mut HashMap<usize, R>, ss: &[String]) -> usize {
    // Replace two rules
    rules.insert(8, R::P(vec![vec![42], vec![42, 8]]));
    rules.insert(11, R::P(vec![vec![42, 31], vec![42, 11, 31]]));

    fn match_rules(rules: &HashMap<usize, R>, s: &[char], stack: &mut Vec<usize>) -> bool {
        if stack.len() == 0 {
            return s.len() == 0;
        }
        if s.len() == 0 {
            return false;
        }
        let rid = stack.pop().unwrap();
        let r = rules.get(&rid).unwrap();
        if rid == 8 || rid == 11 {
            // these are rules with loops.

            // Each production matches a string of length as least 1.

            // Rule 8:
            //
            // Length of string matched by 8 is either 1-k, where k is the
            // length of the remainder of the string. To match x chars, the rule
            // has to be 42,42,42...,x times.
            //
            // Rule 11:
            //
            // Length of string matched by 8 is at least 2

            let saved_stack = stack.clone();
            if rid == 8 {
                for i in 1..s.len() + 1 {
                    let subrule = iter::repeat(42).take(i).collect::<Vec<usize>>();
                    subrule.iter().rev().for_each(|i| stack.push(*i));
                    if match_rules(rules, s, stack) {
                        return true;
                    }
                    *stack = saved_stack.clone();
                }
                return false;
            }
            // rid == 11
            for i in 1..(s.len() / 2 + 1) {
                let subrule = iter::repeat(42)
                    .take(i)
                    .chain(iter::repeat(31).take(i))
                    .collect::<Vec<usize>>();
                subrule.iter().rev().for_each(|i| stack.push(*i));
                if match_rules(rules, s, stack) {
                    return true;
                }
                *stack = saved_stack.clone();
            }
            return false;
        }
        match r {
            R::T(c) => {
                if *c == s[0] {
                    return match_rules(rules, &s[1..], stack);
                }
            }
            R::P(subrules) => {
                let saved_stack = stack.clone();
                for subrule in subrules.iter() {
                    subrule.iter().rev().for_each(|i| stack.push(*i));
                    if match_rules(rules, s, stack) {
                        return true;
                    }
                    *stack = saved_stack.clone();
                }
            }
        }
        false
    }

    ss.iter()
        .filter(|x| match_rules(rules, &x.chars().collect::<Vec<char>>(), &mut vec![0]))
        .count()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let (mut rules, strings) = parse_input(&v);
    println!("{}", solve1(&rules, strings));
    println!("{}", solve2(&mut rules, strings));
    Ok(())
}
