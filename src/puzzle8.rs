use std::convert::TryFrom;
use std::fs;

#[derive(Debug)]
struct Instruction {
    name: String,
    val: i32,
}

#[allow(dead_code)]
pub(crate) fn solution() -> i32 {
    let input = fs::read_to_string("./input/8.txt").unwrap();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            Instruction {
                name: it.next().unwrap().to_owned(),
                val: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    //println!("instructions: {:#?}", instructions);

    let mut acc = 0;
    let mut i = 0;
    let mut visited: Vec<bool> = vec![false; instructions.len()];
    loop {
        if visited[i] {
            break;
        } else {
            visited[i] = true;
        }

        let ins = &instructions[i];
        println!("{:?}", &ins);

        match ins.name.as_str() {
            "acc" => {
                acc += ins.val;
                i += 1
            }
            "jmp" => i = usize::try_from(i as i32 + ins.val).unwrap(),
            "nop" => i += 1,
            _ => panic!("unknown instruction"),
        }
    }

    acc
}

#[allow(dead_code)]
pub(crate) fn solution2() -> i32 {
    let input = fs::read_to_string("./input/8.txt").unwrap();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            Instruction {
                name: it.next().unwrap().to_owned(),
                val: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    //println!("instructions: {:#?}", instructions);

    let acc = 0;
    let i = 0;
    let visited: Vec<bool> = vec![false; instructions.len()];

    let ins = &instructions[i];
    println!("FIRST {:?}", &ins);

    //we must branch
    if ["jmp", "nop"].contains(&instructions[i].name.as_str()) {
        match process(&instructions, &instructions[i], &visited, i, acc, true) {
            Some(acc) => return acc,
            None => {
                return process(
                    &instructions,
                    &flip_ins(&instructions[i]),
                    &visited,
                    i,
                    acc,
                    false,
                )
                .expect("no solution found");
            }
        }
    } else {
        return process(&instructions, &instructions[i], &visited, i, acc, true)
            .expect("no solution found");
    }
}

fn process(
    instructions: &Vec<Instruction>,
    new_ins: &Instruction,
    visited: &Vec<bool>,
    original_i: usize,
    mut acc: i32,
    allow_flip: bool,
) -> Option<i32> {
    let mut visited = visited.clone();
    let mut i = original_i;

    loop {
        if visited[i] {
            return None;
        } else {
            visited[i] = true;
        }

        let ins = if i == original_i {
            new_ins
        } else {
            &instructions[i]
        };
        println!("i:{} {:?}", i, &ins);

        match ins.name.as_str() {
            "acc" => {
                acc += ins.val;
                i += 1
            }
            "jmp" => i = usize::try_from(i as i32 + ins.val).unwrap(),
            "nop" => i += 1,
            _ => panic!("unknown instruction"),
        }

        //check next instruction
        if i >= instructions.len() {
            return Some(acc);
        }

        //we must branch
        if allow_flip && ["jmp", "nop"].contains(&instructions[i].name.as_str()) {
            match process(&instructions, &instructions[i], &visited, i, acc, true) {
                Some(acc) => return Some(acc),
                None => {
                    return process(
                        &instructions,
                        &flip_ins(&instructions[i]),
                        &visited,
                        i,
                        acc,
                        false,
                    );
                }
            }
        }
    }
}

fn flip_ins(ins: &Instruction) -> Instruction {
    println!("REV: {:?}", ins);
    match ins.name.as_str() {
        "nop" => Instruction {
            name: String::from("jmp"),
            val: ins.val,
        },
        "jmp" => Instruction {
            name: String::from("nop"),
            val: ins.val,
        },
        _ => panic!("cant invert ins"),
    }
}
