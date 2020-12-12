use std::fs;
#[derive(Debug)]
struct Nav {
    direction: char,
    amount: i32,
}
const COORDS: [char; 4] = ['E', 'S', 'W', 'N'];

#[allow(dead_code)]
pub(crate) fn solution() -> i32 {
    let input = fs::read_to_string("./input/12.txt").unwrap();
    let navs = input
        .lines()
        .map(|l| {
            let chars = l.chars().collect::<Vec<_>>();
            Nav {
                direction: chars[0],
                amount: chars[1..].iter().collect::<String>().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut direction: usize = 0; //east
    let mut x = 0;
    let mut y = 0;
    for nav in navs {
        println!("{:#?}", nav);
        match nav.direction {
            'R' => {
                let amount = nav.amount / 90;
                direction += amount as usize;
                direction %= 4;
                println!("new direction: {}", COORDS[direction]);
            }
            'L' => {
                let amount = nav.amount / 90;
                direction = (direction + 4) - amount as usize;
                direction %= 4;
                println!("new direction: {}", COORDS[direction]);
            }
            'F' => match COORDS[direction] {
                'N' => y += nav.amount,
                'S' => y -= nav.amount,
                'E' => x += nav.amount,
                'W' => x -= nav.amount,
                _ => panic!("wrong direction"),
            },
            _ => match nav.direction {
                'N' => y += nav.amount,
                'S' => y -= nav.amount,
                'E' => x += nav.amount,
                'W' => x -= nav.amount,
                _ => panic!("wrong direction"),
            },
        };
        println!("location x:{} y:{}", x, y);
    }
    println!("x:{} y:{}", x, y);

    x.abs() + y.abs()
}

#[allow(dead_code)]
pub(crate) fn solution2() -> i32 {
    let input = fs::read_to_string("./input/12.txt").unwrap();
    let navs = input
        .lines()
        .map(|l| {
            let chars = l.chars().collect::<Vec<_>>();
            Nav {
                direction: chars[0],
                amount: chars[1..].iter().collect::<String>().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut x = 10;
    let mut y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;
    for nav in navs {
        println!("{:#?}", nav);
        match nav.direction {
            'R' => {
                let times = nav.amount / 90;
                for _ in 0..times {
                    let tmp=x;
                    x = y;
                    y = tmp * -1;
                }
            }
            'L' => {
                let times = nav.amount / 90;
                for _ in 0..times {
                    let tmp=x;
                    x = y * -1;
                    y = tmp;
                }
            }
            //move ship
            'F' => {
                ship_x += x * nav.amount;
                ship_y += y * nav.amount;
            }
            //move waypoint
            _ => match nav.direction {
                'N' => y += nav.amount,
                'S' => y -= nav.amount,
                'E' => x += nav.amount,
                'W' => x -= nav.amount,
                _ => panic!("wrong direction"),
            },
        };
        println!("waypoint location x:{} y:{}", x, y);
        println!("ship location x:{} y:{}", ship_x, ship_y);
    }

    ship_x.abs() + ship_y.abs()
}
