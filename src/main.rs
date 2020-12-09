use std::{thread, time::Duration};

use crate::puzzle9::*;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;

fn main() {
    println!("START");

    let result = solution2();
    println!("Result: {:?}", result);

    //fix for VSCode
    thread::sleep(Duration::from_millis(100));
}
