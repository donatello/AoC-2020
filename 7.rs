use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Bag {
    col: String,
    contains: HashMap<String, i64>,
}

fn mk_bag(s: &str) -> Bag {
    let spec: Vec<&str> = s.splitn(2, " bags contain ").collect();
    if spec[1] == "no other bags." {
        return Bag {
            col: spec[0].to_string(),
            contains: HashMap::new(),
        };
    }
    let cs: Vec<&str> = spec[1].split(", ").collect();
    let cmap: HashMap<String, i64> = cs
        .iter()
        .map(|t| {
            let ps: Vec<&str> = t.splitn(2, " ").collect();
            let n = ps[0].parse::<i64>().map_or(0, |x| x);
            let col: Vec<&str> = ps[1].split(" bag").take(1).collect();
            (col[0].to_string(), n)
        })
        .collect();
    Bag {
        col: spec[0].to_string(),
        contains: cmap,
    }
}

fn mk_graph_1(bags: &Vec<Bag>) -> (Vec<Vec<usize>>, HashMap<String, usize>) {
    let mut g = vec![vec![]; bags.len()];
    let h: HashMap<String, usize> = bags
        .iter()
        .enumerate()
        .map(|(i, b)| (b.col.clone(), i))
        .collect();
    bags.iter().for_each(|b| {
        let i = h.get(&b.col).unwrap();
        b.contains.keys().for_each(|x| {
            let c = h.get(x).unwrap();
            g[*c].push(*i);
        });
    });
    (g, h)
}

fn solve1(bags: &Vec<Bag>) -> i64 {
    let (g, h) = mk_graph_1(bags);
    let mut has_shiny_gold = vec![false; bags.len()];
    let mut vis = vec![false; bags.len()];
    let mut q = VecDeque::new();
    q.push_back(*h.get("shiny gold").unwrap());
    while !q.is_empty() {
        let c = q.pop_front().unwrap();
        vis[c] = true;
        g[c].iter().for_each(|n| {
            has_shiny_gold[*n] = true;
            q.push_back(*n);
        });
    }
    has_shiny_gold
        .iter()
        .filter(|&x| *x)
        .count()
        .try_into()
        .unwrap()
}

fn solve2(bags: &Vec<Bag>) -> i64 {
    let h: HashMap<String, usize> = bags
        .iter()
        .enumerate()
        .map(|(i, b)| (b.col.clone(), i))
        .collect();
    fn rec(bags: &Vec<Bag>, h: &HashMap<String, usize>, i: usize) -> i64 {
        if bags[i].contains.len() == 0 {
            return 0;
        }
        bags[i]
            .contains
            .iter()
            .map(|(k, v)| {
                let n = h.get(k).unwrap();
                v + v * rec(bags, h, *n)
            })
            .sum()
    }
    rec(bags, &h, *h.get("shiny gold").unwrap())
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let bags: Vec<Bag> = v.iter().map(|x| mk_bag(x)).collect();
    println!("{}", solve1(&bags));
    println!("{}", solve2(&bags));
    Ok(())
}
