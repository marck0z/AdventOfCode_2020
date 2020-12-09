use std::fs;

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
