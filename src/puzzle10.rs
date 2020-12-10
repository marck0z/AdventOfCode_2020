use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub(crate) fn solution() -> u64 {
    let input = fs::read_to_string("./input/10.txt").unwrap();

    let mut volts = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    volts.sort_unstable();

    let mut last = 0;
    let mut diff1 = 0;
    let mut diff3 = 1;
    for volt in volts {
        if volt - last == 1 {
            diff1 += 1;
        } else if volt - last == 3 {
            diff3 += 1;
        }
        last = volt;
    }

    println!("diff1: {} diff3: {}", diff1, diff3);

    diff1 * diff3
}

#[allow(dead_code)]
pub(crate) fn solution2() -> u64 {
    let input = fs::read_to_string("./input/10.txt").unwrap();

    let mut volts = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    volts.sort_unstable();

    combinations(&volts, 0, &mut HashMap::new()) + 1
}

fn combinations(volts: &[u64], start: u64, mut lookup: &mut HashMap<(u64, u64), u64>) -> u64 {
    //memoization, lookup table
    //the only variables that matter is the start and the first element of this subarray
    if let Some(r) = lookup.get(&(start, volts[0])) {
        //println!("using cache for start:{}  first: {}", start, volts[0]);
        return *r;
    }

    let mut result = 0;

    for i in 0..(volts.len() - 1) {
        let previous = if i == 0 { start } else { volts[i - 1] };

        //check if we can remove the current adapter
        if volts[i + 1] - previous <= 3 {
            let mut new_combs = 1;
            //subcombinations
            new_combs += combinations(&volts[i + 1..], previous, &mut lookup);
            result += new_combs;
        }
    }

    //cache the result
    lookup.insert((start, volts[0]), result);

    result
}
