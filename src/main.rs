use std::{time::Duration, thread};

use crate::puzzle::*;

mod puzzle;

fn main() {
    println!("START");

    let result = puzzle7_1();
    println!("Result: {:?}", result);

    print!("WTF DUDE");
    
    thread::sleep(Duration::from_millis(100));
}
