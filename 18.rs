// use std::convert::TryInto;
// use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
enum T {
    Num(i64),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

fn parse_terms(s: &str) -> Vec<T> {
    s.split(' ')
        .flat_map(|x| match x {
            "+" => vec![T::Add],
            "*" => vec![T::Mul],
            _ => {
                let mut v = Vec::new();
                let mut xcopy = x.clone();
                while xcopy.starts_with("(") {
                    v.push(T::OpenParen);
                    xcopy = xcopy.strip_prefix("(").unwrap();
                }
                let mut close_count = 0;
                while xcopy.ends_with(")") {
                    close_count += 1;
                    xcopy = xcopy.strip_suffix(")").unwrap();
                }
                let n = xcopy.parse::<i64>().unwrap();
                v.push(T::Num(n));
                for _ in 0..close_count {
                    v.push(T::CloseParen);
                }
                v
            }
        })
        .collect::<Vec<T>>()
}

fn solve1(v: &Vec<Vec<T>>) -> i64 {
    fn eval(v: &Vec<T>) -> i64 {
        let mut stack: Vec<T> = Vec::new();
        v.iter().cloned().for_each(|t| {
            if stack.len() == 0 || t == T::Mul || t == T::Add || t == T::OpenParen {
                stack.push(t);
            } else if let T::Num(n) = t {
                let op = stack.pop().unwrap();
                if op == T::OpenParen {
                    stack.push(op);
                    stack.push(t);
                } else {
                    let t1 = stack.pop().unwrap();
                    match (op, t1) {
                        (T::Mul, T::Num(n1)) => {
                            stack.push(T::Num(n * n1));
                        }
                        (T::Add, T::Num(n1)) => {
                            stack.push(T::Num(n + n1));
                        }
                        _ => unreachable!(),
                    }
                }
            } else if t == T::CloseParen {
                let n1 = stack.pop().unwrap();
                let open_paren = stack.pop().unwrap();
                assert_eq!(open_paren, T::OpenParen);
                if stack.len() > 0 {
                    let op1 = stack.pop().unwrap();
                    if op1 == T::Mul || op1 == T::Add {
                        let n2 = stack.pop().unwrap();
                        match (n1, n2, op1) {
                            (T::Num(v1), T::Num(v2), T::Mul) => stack.push(T::Num(v1 * v2)),
                            (T::Num(v1), T::Num(v2), T::Add) => stack.push(T::Num(v1 + v2)),
                            _ => unreachable!(),
                        }
                    } else {
                        stack.push(op1);
                        stack.push(n1);
                    }
                } else {
                    stack.push(n1);
                }
            }
        });
        assert_eq!(stack.len(), 1);
        if let T::Num(res) = stack[0] {
            return res;
        } else {
            unreachable!();
        }
    }

    v.iter().map(|ts| eval(&ts)).sum()
}

fn solve2(v: &Vec<Vec<T>>) -> i64 {
    fn eval(v: &Vec<T>) -> i64 {
        let mut stack: Vec<T> = Vec::new();
        v.iter().cloned().for_each(|t| {
            if stack.len() == 0 || t == T::Add || t == T::OpenParen {
                stack.push(t);
            } else if t == T::Mul {
                // stack can be:
                // [n]           or
                // [...,n,*,n]   or
                // [.....,(,n]

                // reduction case is [...,n,*,n]; in all other cases push * onto
                // stack.
                let prev1 = stack.pop().unwrap();
                let prev2 = stack.pop();
                let prev3 = stack.pop();
                if let (Some(T::Num(n1)), Some(T::Mul), T::Num(n2)) = (prev3, prev2, prev1) {
                    stack.push(T::Num(n1 * n2));
                    stack.push(t);
                } else {
                    if let Some(x) = prev3 {
                        stack.push(x);
                    }
                    if let Some(x) = prev2 {
                        stack.push(x);
                    }
                    stack.push(prev1);
                    stack.push(t);
                }
            } else if let T::Num(n) = t {
                // stack can be:
                // [(] or
                // [...,(] or
                // [...,n,+] or
                // [...,n,*]
                //
                // only reduction case is [...,n,+]
                let prev1 = stack.pop().unwrap();
                let prev2 = stack.pop();
                if let (Some(T::Num(n1)), T::Add) = (prev2, prev1) {
                    stack.push(T::Num(n1 + n));
                } else {
                    if let Some(x) = prev2 {
                        stack.push(x);
                    }
                    stack.push(prev1);
                    stack.push(t);
                }
            } else if t == T::CloseParen {
                // stack can be:
                // [(,n]       or
                // [(,(,n] or
                // [...,(,(,n] or
                // [...,n,+,(,n] or
                // [...,n,*,(,n] or
                // [...,(,n,*,n] or
                // [...,(,n,*,n,+,n]
                let n = if let T::Num(n) = stack.pop().unwrap() {
                    n
                } else {
                    unreachable!();
                };
                let prev2 = stack.pop().unwrap();
                let prev3 = stack.pop();
                let prev4 = stack.pop();
                let prev5 = stack.pop();
                let prev6 = stack.pop();
                match (prev6, prev5, prev4, prev3, prev2) {
                    (
                        Some(T::OpenParen),
                        Some(T::Num(n1)),
                        Some(T::Mul),
                        Some(T::Num(n2)),
                        T::Add,
                    ) => stack.push(T::Num(n1 * (n2 + n))),
                    (_, _, Some(T::OpenParen), Some(T::Num(n1)), T::Mul) => {
                        if let Some(x) = prev6 {
                            stack.push(x);
                        }
                        if let Some(x) = prev5 {
                            stack.push(x);
                        }
                        stack.push(T::Num(n1 * n));
                    }
                    (_, _, _, _, T::OpenParen) => {
                        if let Some(x) = prev6 {
                            stack.push(x);
                        }
                        if let Some(x) = prev5 {
                            stack.push(x);
                        }
                        if let Some(x) = prev4 {
                            stack.push(x);
                        }
                        if let Some(x) = prev3 {
                            stack.push(x);
                        }
                        stack.push(T::Num(n));
                    }
                    _ => unreachable!(),
                }
                // After matching the open paren, we may have one reduction to
                // do in the stack: [...,n,+,n]
                let prev1 = stack.pop();
                let prev2 = stack.pop();
                let prev3 = stack.pop();
                if let (Some(T::Num(n1)), Some(T::Add), Some(T::Num(n2))) = (prev3, prev2, prev1) {
                    stack.push(T::Num(n1 + n2));
                } else {
                    if let Some(x) = prev3 {
                        stack.push(x);
                    }
                    if let Some(x) = prev2 {
                        stack.push(x);
                    }
                    if let Some(x) = prev1 {
                        stack.push(x);
                    }
                }
            }
        });
        let prev1 = stack.pop();
        let prev2 = stack.pop();
        let prev3 = stack.pop();
        let prev4 = stack.pop();
        let prev5 = stack.pop();
        match (prev5, prev4, prev3, prev2, prev1) {
            (Some(T::Num(n1)), Some(T::Mul), Some(T::Num(n2)), Some(T::Add), Some(T::Num(n3))) => {
                n1 * (n2 + n3)
            }
            (None, None, Some(T::Num(n1)), Some(T::Mul), Some(T::Num(n2))) => n1 * n2,
            (None, None, None, None, Some(T::Num(n2))) => n2,
            _ => unreachable!(),
        }
    }

    v.iter().map(|ts| eval(&ts)).sum()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let terms: Vec<Vec<T>> = v.iter().map(|x| parse_terms(x)).collect();
    println!("{}", solve1(&terms));
    println!("{}", solve2(&terms));
    Ok(())
}
