use std::{collections::HashMap, fs};

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
