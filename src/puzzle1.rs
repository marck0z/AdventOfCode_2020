use std::fs;
use std::path::PathBuf;

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
