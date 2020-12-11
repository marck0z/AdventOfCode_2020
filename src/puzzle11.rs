use std::fs;

#[allow(dead_code)]
pub(crate) fn solution() -> i32 {
    let input = fs::read_to_string("./input/11.txt").unwrap();
    let mut seats = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    //rows
    for i in 0..seats.len() {
        //columns
        for j in 0..seats[i].len() {
            print!("{}", seats[i][j]);
        }
        print!("\n")
    }

    let mut round = 0;
    loop {
        let mut changed = false;

        let mut new_seats = seats.clone();
        //rows
        for i in 0..seats.len() {
            //columns
            for j in 0..seats[i].len() {
                match seats[i][j] {
                    'L' => {
                        if get_occupied_seats(&seats, i, j) == 0 {
                            new_seats[i][j] = '#';
                            changed = true;
                        }
                    }
                    '#' => {
                        if get_occupied_seats(&seats, i, j) >= 4 {
                            new_seats[i][j] = 'L';
                            changed = true;
                        }
                    }
                    '.' => {}
                    _ => panic!("wrong character"),
                }
            }
        }

        round += 1;
        seats = new_seats;
        println!("\n\nround {}", round);
        for i in 0..seats.len() {
            for j in 0..seats[i].len() {
                print!("{}", seats[i][j]);
            }
            print!("\n")
        }

        if !changed {
            break;
        }
    }

    //count how many occupied
    seats
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count() as i32)
        .sum()
}

fn get_occupied_seats(seats: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut occupied = 0;
    //left
    if j > 0 && seats[i][j - 1] == '#' {
        occupied += 1;
    }

    //right
    if j + 1 < seats[i].len() && seats[i][j + 1] == '#' {
        occupied += 1;
    }

    //up
    if i > 0 && seats[i - 1][j] == '#' {
        occupied += 1;
    }

    //down
    if i + 1 < seats.len() && seats[i + 1][j] == '#' {
        occupied += 1;
    }

    //upper left
    if i > 0 && j > 0 && seats[i - 1][j - 1] == '#' {
        occupied += 1;
    }

    //upper right
    if i > 0 && j + 1 < seats[i].len() && seats[i - 1][j + 1] == '#' {
        occupied += 1;
    }

    //lower left
    if i + 1 < seats.len() && j > 0 && seats[i + 1][j - 1] == '#' {
        occupied += 1;
    }

    //lower right
    if i + 1 < seats.len() && j + 1 < seats[i].len() && seats[i + 1][j + 1] == '#' {
        occupied += 1;
    }

    occupied
}
