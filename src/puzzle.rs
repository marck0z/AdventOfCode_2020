use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
struct Bag {
    name: String,
    subbags: Vec<Rc<RefCell<Bag>>>,
}

pub(crate) fn puzzle7_1() -> i32 {
    let input = fs::read_to_string("./input/7.txt").unwrap();

    let bag_regex = Regex::new(r"([a-z ]+) bags contain").unwrap();
    let subs_regex = Regex::new(r"(\d+) ([a-z\s]+) bags*[,.]").unwrap();

    let mut index: Vec<Rc<RefCell<Bag>>> = Vec::new();

    for l in input.lines() {
        let bag_caps = bag_regex.captures(l).unwrap();
        let bag_name = bag_caps.get(1).unwrap().as_str();
        println!("bag name: {}", bag_name);

        let bag = get_bag(bag_name, &mut index);

        for cap in subs_regex.captures_iter(l) {
            let amount = &cap[1];
            let sub = &cap[2];
            println!("quantity: {} subbag: {}", amount, sub);

            let subbag = get_bag(sub, &mut index);
            bag.borrow_mut().subbags.push(subbag);
        }
    }

    for bag in index {
        println!("{:#?}", bag.borrow());
    }

    0
}

fn get_bag(name: &str, index: &mut Vec<Rc<RefCell<Bag>>>) -> Rc<RefCell<Bag>> {
    match index.iter().find(|b| b.borrow().name == name) {
        Some(bag) => Rc::clone(bag),
        None => {
            let bag = Rc::new(RefCell::new(Bag {
                name: name.to_owned(),
                subbags: vec![],
            }));

            index.push(Rc::clone(&bag));

            Rc::clone(&bag)
        }
    }
}

#[allow(dead_code)]
pub(crate) fn puzzle6_2() -> usize {
    let input = fs::read_to_string("./input/6.txt").unwrap();
    input
        .split("\n\n")
        .map(|g| {
            let res: Vec<char> = ('a'..='z').collect();
            g.lines()
                .fold(res, |mut acc, e| {
                    acc.retain(|c| e.contains(*c));
                    acc
                })
                .len()
        })
        .sum()
}

#[allow(dead_code)]
pub(crate) fn puzzle6() -> usize {
    let input = fs::read_to_string("./input/6.txt").unwrap();
    input
        .split("\n\n")
        .map(|g| {
            let mut chars = g.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
            chars.sort_unstable();
            chars.dedup();
            chars.len()
        })
        .sum()
}

#[allow(dead_code)]
pub(crate) fn puzzle5_2() -> i32 {
    let input = fs::read_to_string("./input/5.txt").unwrap();

    let mut ids = input
        .lines()
        .map(|l| get_row(&l[..7]) * 8 + get_column(&l[7..]))
        .collect::<Vec<_>>();

    ids.sort();

    let mut idx = ids[0];

    for id in ids {
        println!("id: {}", id);
        println!("id: {}", id);
        println!("id: {}", id);
        if id != idx {
            break;
        } else {
            idx += 1
        }
    }

    idx
}

#[allow(dead_code)]
pub(crate) fn puzzle5() -> i32 {
    let input = fs::read_to_string("./input/5.txt").unwrap();

    input
        .lines()
        .map(|l| get_row(&l[..7]) * 8 + get_column(&l[7..]))
        .max()
        .unwrap()
}

fn get_column(input: &str) -> i32 {
    let mut min = 0;
    let mut max = 7;

    for c in input.chars() {
        let half = (max - min) / 2 + 1;

        if c == 'L' {
            max -= half;
        }
        if c == 'R' {
            min += half;
        }
    }

    //println!("column: {}", min);
    min
}

fn get_row(input: &str) -> i32 {
    let mut min = 0;
    let mut max = 127;

    for c in input.chars() {
        let half = (max - min) / 2 + 1;

        if c == 'F' {
            max -= half;
        }
        if c == 'B' {
            min += half;
        }
    }

    //println!("row: {}", min);
    min
}

#[allow(dead_code)]
pub(crate) fn puzzle4_2() -> usize {
    let input = fs::read_to_string("./input/4.txt").unwrap();

    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    input
        .split("\n\n")
        .filter(|p| {
            let kvs: HashMap<&str, &str> = p
                .split(|c| c == '\n' || c == ' ')
                .map(|kv: &str| {
                    let mut kv = kv.split(':');
                    (kv.next().unwrap(), kv.next().unwrap())
                })
                .collect();
            if !required_keys.iter().all(|key| kvs.contains_key(key)) {
                return false;
            }

            kvs.iter().all(|(k, v)| {
                if *k == "byr" {
                    v.len() == 4 && {
                        match v.parse::<i32>() {
                            Ok(year) => year >= 1920 && year <= 2002,
                            Err(_) => false,
                        }
                    }
                } else if *k == "iyr" {
                    v.len() == 4 && {
                        match v.parse::<i32>() {
                            Ok(year) => year >= 2010 && year <= 2020,
                            Err(_) => false,
                        }
                    }
                } else if *k == "eyr" {
                    v.len() == 4 && {
                        match v.parse::<i32>() {
                            Ok(year) => year >= 2020 && year <= 2030,
                            Err(_) => false,
                        }
                    }
                } else if *k == "hgt" {
                    if v.len() < 3 {
                        false
                    } else {
                        let (h, u) = v.split_at(v.len() - 2);
                        if u == "cm" {
                            match h.parse::<i32>() {
                                Ok(height) => height >= 150 && height <= 193,
                                Err(_) => false,
                            }
                        } else if u == "in" {
                            match h.parse::<i32>() {
                                Ok(height) => height >= 59 && height <= 76,
                                Err(_) => false,
                            }
                        } else {
                            false
                        }
                    }
                } else if *k == "hcl" {
                    v.len() == 7 && {
                        let chars = v.chars().collect::<Vec<_>>();
                        if chars[0] != '#' {
                            false
                        } else {
                            chars[1..]
                                .iter()
                                .all(|c| (*c >= 'a' && *c <= 'f') || (*c >= '0' && *c <= '9'))
                        }
                    }
                } else if *k == "ecl" {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v)
                } else if *k == "pid" {
                    v.len() == 9 && v.chars().all(|c| c >= '0' && c <= '9')
                } else if *k == "cid" {
                    true
                } else {
                    panic!("invalid key:{}", k)
                }
            })
        })
        .count()
}

#[allow(dead_code)]
pub(crate) fn puzzle4() -> usize {
    let input = fs::read_to_string("./input/4.txt").unwrap();

    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    input
        .split("\n\n")
        .filter(|p| {
            let keys: Vec<&str> = p
                .split(|c| c == '\n' || c == ' ')
                .map(|kv| kv.split(':').next().unwrap())
                .collect();
            required_keys.iter().all(|key| keys.contains(key))
        })
        .count()
}

#[allow(dead_code)]
pub(crate) fn puzzle3_1() -> i32 {
    let input = fs::read_to_string("./input/3.txt").unwrap();

    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    //height (outer array) down
    let mut i = 0;
    //width (inner array) right
    let mut j = 0;

    let right = 1;
    let down = 2;

    let mut res = 0;
    loop {
        i += down;
        j += right;

        if i >= map.len() {
            break;
        }
        if j >= map[i].len() {
            j -= map[i].len()
        }
        if map[i][j] {
            res += 1;
        }
    }

    res
}

#[allow(dead_code)]
pub(crate) fn puzzle2_2() -> usize {
    let input = fs::read_to_string("./input/2.txt").unwrap();

    input
        .lines()
        .filter(|l| {
            let blocks: Vec<&str> = l.split_whitespace().collect();
            let (min, max) = {
                let range: Vec<usize> = blocks[0].split('-').map(|c| c.parse().unwrap()).collect();
                (range[0], range[1])
            };
            let letter = blocks[1].chars().next().unwrap();
            let pass: Vec<char> = blocks[2].chars().collect();

            if pass[min - 1] == letter {
                pass[max - 1] != letter
            } else {
                pass[max - 1] == letter
            }
        })
        .count()
}

#[allow(dead_code)]
pub(crate) fn puzzle2_1() -> usize {
    let input = fs::read_to_string("./input/2.txt").unwrap();

    input
        .lines()
        .filter(|l| {
            let blocks: Vec<&str> = l.split_whitespace().collect();
            let (min, max) = {
                let range: Vec<usize> = blocks[0].split('-').map(|c| c.parse().unwrap()).collect();
                (range[0], range[1])
            };
            let letter = blocks[1].chars().next().unwrap();
            let pass = blocks[2];

            let count = pass.chars().filter(|c| *c == letter).count();

            count <= max && count >= min
        })
        .count()
}

#[allow(dead_code)]
pub(crate) fn puzzle1_1() -> Option<i32> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join("1.txt");
    let input = fs::read_to_string(path).expect("Something went wrong reading the input file");

    let numbers = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let twin = find_twin(2020, &numbers);
    twin.map(|t| t * (2020 - t))
}

#[allow(dead_code)]
pub(crate) fn puzzle1_2() -> Option<i32> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join("1.txt");
    let input = fs::read_to_string(path).expect("Something went wrong reading the input file");

    let numbers = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for (i, n) in numbers.iter().enumerate() {
        let bro = 2020 - *n;
        println!("og: {} bro: {}", n, bro);
        if let Some(twin) = find_twin(bro, &numbers[i + 1..]) {
            println!("found twins: {} and {}", twin, bro - twin);
            return Some(*n * twin * (bro - twin));
        }
    }

    None
}

fn find_twin(bro: i32, numbers: &[i32]) -> Option<i32> {
    let mut twins = Vec::with_capacity(numbers.len());
    for n in numbers {
        let twin = bro - *n;
        if twins.contains(n) {
            return Some(twin);
        } else {
            twins.push(twin);
        }
    }

    None
}
