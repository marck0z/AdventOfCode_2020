use std::{thread, time::Duration};

use crate::puzzle11_2::*;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;
mod puzzle10;
mod puzzle11;
mod puzzle11_2;

fn main() {
    println!("START");

    let result = solution();
    println!("Result: {:?}", result);

    //fix for VSCode
    thread::sleep(Duration::from_millis(100));
}
