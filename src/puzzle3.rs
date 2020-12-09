use std::fs;

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
