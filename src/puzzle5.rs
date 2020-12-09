use std::fs;

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
