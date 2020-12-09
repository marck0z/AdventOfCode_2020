use std::fs;

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
