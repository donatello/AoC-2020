// use std::convert::TryInto;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};
//use std::iter;

struct Tile {
    pic: Vec<String>,
    border: Vec<u16>,
}

fn rotate(v: &Vec<u16>) -> Vec<u16> {
    vec![v[3], v[0], v[1], v[2]]
}

fn reverse_side(mut u: u16) -> u16 {
    let mut r = 0;
    for _ in 0..10 {
        r = (r << 1) + u % 2;
        u = u >> 1;
    }
    r
}

fn flip(v: &Vec<u16>) -> Vec<u16> {
    vec![
        reverse_side(v[0]),
        reverse_side(v[3]),
        reverse_side(v[2]),
        reverse_side(v[1]),
    ]
}

fn rotate_many(v: &Vec<u16>, c: u8) -> Vec<u16> {
    let mut r = v.clone();
    for _ in 0..c {
        r = rotate(&r);
    }
    r
}

fn rotate_pic(v: &Vec<String>) -> Vec<String> {
    let n = v.len();
    let mut out = vec![vec!['.'; n]; n];
    for i in 0..n {
        for (j, c) in v[i].chars().enumerate() {
            out[j][n - 1 - i] = c;
        }
    }
    out.iter().map(|r| r.iter().collect::<String>()).collect()
}

fn rotate_pic_many(v: &Vec<String>, c: u8) -> Vec<String> {
    let mut r = v.clone();
    for _ in 0..c {
        r = rotate_pic(&r)
    }
    r
}

fn flip_pic(v: &Vec<String>) -> Vec<String> {
    v.iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect()
}

impl Tile {
    fn from_pic(v: Vec<String>) -> Self {
        let n = v.len();
        fn byte2bit(c: u8) -> u16 {
            match c {
                b'.' => 0,
                b'#' => 1,
                _ => unreachable!(),
            }
        }
        let top = v[0].bytes().map(byte2bit).fold(0, |acc, x| acc * 2 + x);
        let right = (0..n)
            .into_iter()
            .map(|i| v[i].as_bytes()[n - 1])
            .map(byte2bit)
            .fold(0, |acc, x| acc * 2 + x);
        let bottom = (0..n)
            .into_iter()
            .rev()
            .map(|i| v[n - 1].as_bytes()[i])
            .map(byte2bit)
            .fold(0, |acc, x| acc * 2 + x);
        let left = (0..n)
            .into_iter()
            .rev()
            .map(|i| v[i].as_bytes()[0])
            .map(byte2bit)
            .fold(0, |acc, x| acc * 2 + x);
        Tile {
            pic: v,
            border: vec![top, right, bottom, left],
        }
    }

    fn get_sides_for_orientation(&self, o: u8) -> Vec<u16> {
        if o < 4 {
            rotate_many(&self.border, o)
        } else {
            let f = flip(&self.border);
            let c = o - 4;
            rotate_many(&f, c)
        }
    }

    fn get_pic(&self, o: u8) -> Vec<String> {
        if o < 4 {
            rotate_pic_many(&self.pic, o)
        } else {
            let f = flip_pic(&self.pic);
            rotate_pic_many(&f, o - 4)
        }
    }
}

fn drop_borders(v: &Vec<String>) -> Vec<String> {
    let n = v.len();
    v.iter()
        .skip(1)
        .take(n - 2)
        .map(|s| s.chars().skip(1).take(n - 2).collect::<String>())
        .collect()
}

fn parse_input(v: Vec<String>) -> HashMap<u64, Tile> {
    let tiles = v.into_iter().fold(vec![vec![]], |mut acc, x| {
        if x == "" {
            acc.push(vec![]);
        } else {
            let n = acc.len();
            acc[n - 1].push(x);
        }
        acc
    });
    let tile_pics = tiles
        .into_iter()
        .filter(|x| x.len() > 0)
        .map(|t| {
            let tid = t[0]
                .split(&[' ', ':'][..])
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let tile = t[1..].into_iter().cloned().collect::<Vec<String>>();
            (tid, tile)
        })
        .collect::<HashMap<u64, Vec<String>>>();

    tile_pics
        .iter()
        .map(|(id, v)| (id.clone(), Tile::from_pic(v.clone())))
        .collect::<HashMap<u64, Tile>>()
}

fn get_corner_ids(tile_map: &HashMap<u64, Tile>) -> Vec<u64> {
    let side_match_count: HashMap<u64, u64> = tile_map
        .iter()
        .map(|(id, t)| {
            let mut side_count = 0;
            for i in 0..4 {
                for (o_id, o_sides) in tile_map.iter() {
                    if id == o_id {
                        continue;
                    }
                    let mut found = false;
                    for j in 0..4 {
                        if t.border[i] == o_sides.border[j]
                            || t.border[i] == reverse_side(o_sides.border[j])
                        {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        side_count += 1;
                        break;
                    }
                }
            }
            (id.clone(), side_count)
        })
        .collect();

    side_match_count
        .iter()
        .filter(|(_, &x)| x == 2)
        .map(|(id, _)| id.clone())
        .collect()
}

fn solve1(tile_map: &HashMap<u64, Tile>) -> u64 {
    let corners = get_corner_ids(tile_map);
    assert_eq!(corners.len(), 4);
    corners.iter().product()
}

static NBRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_nbrs(i: usize, j: usize, n: usize) -> Vec<Option<(usize, usize)>> {
    NBRS.iter()
        .cloned()
        .map(|(di, dj)| {
            let p = (i as i64 + di, j as i64 + dj);
            if p.0 < 0 || p.0 >= n as i64 || p.1 < 0 || p.1 >= n as i64 {
                None
            } else {
                let p1 = (p.0 as usize, p.1 as usize);
                Some(p1)
            }
        })
        .collect()
}

fn find_monster(pic: &mut Vec<Vec<u8>>) -> bool {
    let monster: Vec<&str> = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let w = monster[0].len();
    let n = pic.len();
    let mut found_monster = false;
    for x in 0..n - 3 {
        for y in 0..n - w {
            let found_here = {
                let mut found = true;
                'outer: for i in 0..3 {
                    for j in 0..w {
                        if monster[i].as_bytes()[j] == b'#' {
                            if pic[x + i][y + j] != b'#' {
                                found = false;
                                break 'outer;
                            }
                        }
                    }
                }
                found
            };
            if found_here {
                found_monster = true;
                for i in 0..3 {
                    for j in 0..w {
                        if monster[i].as_bytes()[j] == b'#' {
                            pic[x + i][y + j] = b'O';
                        }
                    }
                }
            }
        }
    }
    return found_monster;
}

fn solve2(tile_map: &HashMap<u64, Tile>) -> u64 {
    let corners = get_corner_ids(&tile_map);

    let mut sides_index: HashMap<u16, Vec<u64>> = HashMap::new();
    tile_map
        .iter()
        .map(|(id, t)| {
            // 8 possible borders, but we take only distinct ones.
            let s = t
                .border
                .iter()
                .chain(flip(&t.border).iter())
                .cloned()
                .collect::<HashSet<u16>>();
            (id, s)
        })
        .for_each(|(id, s)| {
            s.into_iter().for_each(|b| {
                match sides_index.get_mut(&b) {
                    Some(v) => v.push(*id),
                    None => {
                        sides_index.insert(b, vec![*id]);
                    }
                };
            });
        });

    let non_match_sides = sides_index
        .iter()
        .filter_map(|(side, ids)| {
            if ids.len() == 1 {
                Some(side.clone())
            } else {
                None
            }
        })
        .collect::<HashSet<u16>>();

    // // Printing this below shows that no sides belong to more than 2 tiles,
    // // indicating that no backtracking seems to be needed! (It prints nothing!)
    // sides_index
    //     .iter()
    //     .filter(|(_, s)| s.len() != 2 && s.len() != 1)
    //     .for_each(|p| println!("{}: {}", p.0, p.1.len()));

    let n = (tile_map.len() as f64).sqrt() as usize;
    let mut tpos = vec![vec![0; n]; n];
    let corner_tile = tile_map.get(&corners[0]).unwrap();
    let mut o_map = HashMap::new();

    // orient corners[0] at the top left corner of the jigsaw puzzle.
    tpos[0][0] = corners[0];
    for o in 0..8 {
        let mut is_ok = true;
        for (i, popt) in get_nbrs(0, 0, n).into_iter().enumerate() {
            match popt {
                None => {
                    let s = corner_tile.get_sides_for_orientation(o)[i];
                    if !non_match_sides.contains(&s) {
                        is_ok = false;
                    }
                }
                Some(_) => {}
            }
        }
        if is_ok {
            o_map.insert(corners[0], o);
            break;
        }
    }
    assert_eq!(o_map.len(), 1);

    // Now arrange all tiles!
    for i in 0..n {
        for j in 0..n {
            if i == 0 && j == 0 {
                continue;
            }
            let mut is_above = false;
            let (pi, pj) = if j == 0 {
                is_above = true;
                (i - 1, 0)
            } else {
                (i, j - 1)
            };
            let ptile_id = tpos[pi][pj];
            let ptile = tile_map.get(&ptile_id).unwrap();
            let po = o_map.get(&ptile_id).unwrap();
            let pb = ptile.get_sides_for_orientation(*po);
            let pside = if is_above { pb[2] } else { pb[1] };
            let cside = reverse_side(pside);
            let tiles_with_cside = sides_index
                .get(&cside)
                .unwrap()
                .iter()
                .filter(|&id| *id != ptile_id)
                .cloned()
                .collect::<Vec<u64>>();
            assert_eq!(tiles_with_cside.len(), 1);
            let ctile_id = tiles_with_cside[0];
            let ctile = tile_map.get(&ctile_id).unwrap();
            let os = (0..8)
                .into_iter()
                .filter(|o| {
                    let b = ctile.get_sides_for_orientation(*o);
                    let side = if is_above { b[0] } else { b[3] };
                    side == cside
                })
                .take(1)
                .nth(0)
                .unwrap();
            tpos[i][j] = ctile_id;
            o_map.insert(ctile_id, os);
        }
    }

    let mut pic = vec![String::new(); n * 8];
    for i in 0..n {
        for j in 0..n {
            let id = tpos[i][j];
            let tile = tile_map.get(&id).unwrap();
            let o = o_map.get(&id).unwrap();
            let tile_pic = tile.get_pic(*o);
            let tile_img = drop_borders(&tile_pic);
            for k in 0..8 {
                pic[8 * i + k].push_str(&tile_img[k]);
            }
        }
    }

    // big pic is ready.
    let pic_bytes = (0..8)
        .into_iter()
        .filter_map(|o| {
            let p = if o < 4 {
                rotate_pic_many(&pic, o)
            } else {
                let f = flip_pic(&pic);
                rotate_pic_many(&f, o - 4)
            };
            let mut pic_bytes = p
                .iter()
                .map(|s| s.bytes().collect::<Vec<u8>>())
                .collect::<Vec<Vec<u8>>>();
            if find_monster(&mut pic_bytes) {
                Some(pic_bytes)
            } else {
                None
            }
        })
        .take(1)
        .nth(0)
        .unwrap();

    pic_bytes
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&x| *x == b'#')
        .count() as u64
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let v: Vec<String> = handle.lines().collect::<Result<Vec<String>, _>>()?;
    let tiles = parse_input(v);
    println!("{}", solve1(&tiles));
    println!("{}", solve2(&tiles));
    Ok(())
}
