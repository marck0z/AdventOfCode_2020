use std::fs;

const PREAMBLE: usize = 25;

#[allow(dead_code)]
pub(crate) fn solution2() -> u64 {
    let input = fs::read_to_string("./input/9.txt").unwrap();

    let numbers: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    println!("{:?}", numbers);

    let mut start: usize = 0;
    let mut end: usize = start + PREAMBLE;

    let invalid_n = loop {
        if is_invalid(&numbers[start..end], numbers[end]) {
            break numbers[end];
        }
        start += 1;
        end += 1;
    };

    //find the second part of the problem
    for i in 0..numbers.len() - 1 {
        if let Some(end) = find_contiguos(&numbers[i..], invalid_n) {
            let seq = &numbers[i..=(i + end)];
            println!("result seq: {:#?}", seq);

            return seq.iter().max().unwrap() + seq.iter().min().unwrap();
        }
    }

    panic!("result not found")
}

fn find_contiguos(numbers: &[u64], objective: u64) -> Option<usize> {
    let mut buffer = 0;

    for i in 0..numbers.len() {
        if numbers[i] == objective {
            return None;
        }

        buffer += numbers[i];
        if buffer == objective {
            return Some(i);
        } else if buffer > objective {
            return None;
        }
    }

    None
}

#[allow(dead_code)]
pub(crate) fn solution() -> u64 {
    let input = fs::read_to_string("./input/9.txt").unwrap();

    let numbers: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    println!("{:?}", numbers);

    let mut start: usize = 0;
    let mut end: usize = start + PREAMBLE;

    while end < numbers.len() {
        if is_invalid(&numbers[start..end], numbers[end]) {
            return numbers[end];
        }
        start += 1;
        end += 1;
    }

    panic!("number not found");
}

fn is_invalid(numbers: &[u64], objective: u64) -> bool {
    let mut bros: Vec<u64> = Vec::with_capacity(numbers.len());

    for n in numbers {
        if bros.contains(n) {
            return false;
        }
        if objective >= *n {
            bros.push(objective - *n);
        }
    }

    true
}
