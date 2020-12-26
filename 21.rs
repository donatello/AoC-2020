use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};
//use std::iter;

fn parse_input(v: &Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut ret: Vec<(Vec<String>, Vec<String>)> = vec![];
    for line in v.iter() {
        let ps = line
            .split(" (contains ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let ingedients = ps[0]
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let allergens = ps[1]
            .split(&[' ', ',', ')'][..])
            .filter(|x| *x != "")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        ret.push((ingedients, allergens));
    }
    ret
}

fn get_potential_allergens(
    foods: &Vec<(Vec<String>, Vec<String>)>,
) -> (
    Vec<(HashSet<String>, HashSet<String>)>,
    HashSet<String>,
    HashSet<String>,
) {
    let food_sets = foods
        .iter()
        .map(|(is, als)| {
            (
                is.iter().cloned().collect::<HashSet<String>>(),
                als.iter().cloned().collect::<HashSet<String>>(),
            )
        })
        .collect::<Vec<(HashSet<String>, HashSet<String>)>>();
    let allergens = food_sets
        .iter()
        .flat_map(|(_, als)| als.iter())
        .cloned()
        .collect::<HashSet<String>>();
    let allergen_ingredients = allergens
        .iter()
        .map(|a| {
            let foods_containing_a = food_sets
                .iter()
                .filter_map(|(is, als)| if als.contains(a) { Some(is) } else { None })
                .cloned()
                .collect::<Vec<HashSet<String>>>();
            let mut s = foods_containing_a[0].clone();
            for f in foods_containing_a[1..].iter() {
                s = s.intersection(f).cloned().collect();
            }
            s
        })
        .collect::<Vec<HashSet<String>>>();
    let pot_allergens = allergen_ingredients.iter().fold(HashSet::new(), |acc, x| {
        acc.union(x).cloned().collect::<HashSet<String>>()
    });
    (food_sets, allergens, pot_allergens)
}

fn solve1(foods: &Vec<(Vec<String>, Vec<String>)>) -> u64 {
    let (food_sets, _, potential_allergens) = get_potential_allergens(foods);
    food_sets
        .iter()
        .map(|(is, _)| {
            is.difference(&potential_allergens)
                .cloned()
                .collect::<HashSet<String>>()
                .len() as u64
        })
        .sum()
}

fn solve2(foods: &Vec<(Vec<String>, Vec<String>)>) -> String {
    let (food_sets, mut allergens, potential_allergens) = get_potential_allergens(foods);
    let inert_ingredients = food_sets
        .iter()
        .map(|(is, _)| {
            is.difference(&potential_allergens)
                .cloned()
                .collect::<HashSet<String>>()
        })
        .fold(HashSet::new(), |s, x| {
            s.union(&x).cloned().collect::<HashSet<String>>()
        });
    let mut food_sets_2 = food_sets
        .iter()
        .map(|(is, als)| {
            (
                is.difference(&inert_ingredients)
                    .cloned()
                    .collect::<HashSet<String>>(),
                als.clone(),
            )
        })
        .collect::<Vec<(HashSet<String>, HashSet<String>)>>();
    let mut m: HashMap<String, String> = HashMap::new();
    while allergens.len() > 0 {
        let mut found = false;
        allergens.iter().for_each(|allergen| {
            let foods_with_allergen = food_sets_2
                .iter()
                .filter_map(|(is, als)| {
                    if als.contains(allergen) {
                        Some(is)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect::<Vec<HashSet<String>>>();
            let pot_ings_for_allergen = if foods_with_allergen.len() > 1 {
                foods_with_allergen[1..]
                    .iter()
                    .fold(foods_with_allergen[0].clone(), |s, a| {
                        s.intersection(a).cloned().collect::<HashSet<String>>()
                    })
            } else {
                foods_with_allergen[0].clone()
            };
            assert_ne!(pot_ings_for_allergen.len(), 0);
            if pot_ings_for_allergen.len() == 1 {
                m.insert(
                    allergen.clone(),
                    pot_ings_for_allergen.iter().nth(0).unwrap().to_string(),
                );
                found = true;
            }
        });
        if found {
            m.iter().for_each(|(al, ig)| {
                allergens.remove(al);
                food_sets_2.iter_mut().for_each(|(is, _)| {
                    is.remove(ig);
                });
            })
        } else {
            unreachable!();
        }
    }
    let mut a_i_pairs = m
        .iter()
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect::<Vec<(String, String)>>();
    a_i_pairs.sort_unstable_by_key(|(al, _)| al.clone());
    let canonical_ings = a_i_pairs
        .iter()
        .map(|(_, ig)| ig)
        .cloned()
        .collect::<Vec<String>>();
    let mut s = String::new();
    for ing in canonical_ings.iter() {
        s.push_str(ing);
        s.push(',');
    }
    s.pop();
    s
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let foods = parse_input(&v);
    // println!("{:?}", foods);
    println!("{}", solve1(&foods));
    println!("{}", solve2(&foods));
    Ok(())
}
