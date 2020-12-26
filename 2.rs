use std::io::{self, BufRead};

fn is_valid(s: &str) -> bool {
    let v: Vec<&str> = s.split(&['-', ' ', ':'][..]).collect();
    // println!("{:?}", v);
    let low: usize = v[0].parse().unwrap();
    let hi: usize = v[1].parse().unwrap();
    let mut t = v[2].chars();
    let c: char = t.next().unwrap();
    let pw = v[4];

    let count = pw.chars().filter(|x| x == &c).count();
    return count >= low && count <= hi;
}

fn is_valid2(s: &str) -> bool {
    let v: Vec<&str> = s.split(&['-', ' ', ':'][..]).collect();
    let low: usize = v[0].parse().unwrap();
    let hi: usize = v[1].parse().unwrap();
    let mut t = v[2].chars();
    let c: char = t.next().unwrap();
    let pw = v[4];

    let mut ok = false;
    pw.chars().enumerate().for_each(|(i, x)| {
        if i == low - 1 || i == hi - 1 {
            if x == c {
                ok = !ok
            }
        }
    });
    return ok;
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let valid1 = lines.iter().filter(|&x| is_valid(x)).count();
    let valid2 = lines.iter().filter(|&x| is_valid2(x)).count();
    println!("{}\n{}", valid1, valid2);
    Ok(())
}
