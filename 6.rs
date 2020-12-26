use std::collections::HashSet;
use std::convert::TryInto;
use std::io::{self, BufRead};

fn mk_emptyline_sep_records(v: Vec<String>) -> Vec<Vec<String>> {
    fn f(mut acc: Vec<Vec<String>>, item: String) -> Vec<Vec<String>> {
        if item == "" {
            acc.push(vec![]);
        } else {
            let n = acc.len() - 1;
            acc[n].push(item);
        }
        acc
    }
    v.into_iter().fold(vec![vec![]], f)
}

fn solve1(rs: &Vec<Vec<String>>) -> u64 {
    fn ans_count(ans: &Vec<String>) -> u64 {
        ans.iter()
            .map(|x| x.chars().collect::<HashSet<char>>())
            .fold(HashSet::new(), |s, t| s.union(&t).cloned().collect())
            .len()
            .try_into()
            .unwrap()
    }
    rs.iter().map(ans_count).sum()
}

fn solve2(rs: &Vec<Vec<String>>) -> u64 {
    fn ans_count(ans: &Vec<String>) -> u64 {
        let st: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        ans.iter()
            .map(|x| x.chars().collect::<HashSet<char>>())
            .fold(st, |s, t| s.intersection(&t).cloned().collect())
            .len()
            .try_into()
            .unwrap()
    }
    rs.iter().map(ans_count).sum()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let recs = mk_emptyline_sep_records(v);
    println!("{}", solve1(&recs));
    println!("{}", solve2(&recs));
    Ok(())
}
