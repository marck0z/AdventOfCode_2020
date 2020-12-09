use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct BagSol1 {
    name: String,
    containers: Vec<Rc<RefCell<BagSol1>>>,
}

#[allow(dead_code)]
pub(crate) fn puzzle7_1() -> usize {
    let input = fs::read_to_string("./input/7.txt").unwrap();

    let bag_regex = Regex::new(r"([a-z ]+) bags contain").unwrap();
    let subs_regex = Regex::new(r"(\d+) ([a-z\s]+) bags*[,.]").unwrap();

    let mut index: Vec<Rc<RefCell<BagSol1>>> = Vec::new();

    for l in input.lines() {
        let bag_caps = bag_regex.captures(l).unwrap();
        let bag_name = bag_caps.get(1).unwrap().as_str();
        println!("bag name: {}", bag_name);

        let bag = get_bag_sol1(bag_name, &mut index);

        for cap in subs_regex.captures_iter(l) {
            let amount = &cap[1];
            let sub = &cap[2];
            println!("quantity: {} subbag: {}", amount, sub);

            let subbag = get_bag_sol1(sub, &mut index);
            subbag.borrow_mut().containers.push(Rc::clone(&bag));
        }
    }

    /*     for bag in &index {
        println!("{:#?}", bag.borrow());
    } */

    let shiny_gold = index
        .iter()
        .find(|b| b.borrow().name == "shiny gold")
        .unwrap();
    get_containers(Rc::clone(shiny_gold)).len()
}

fn get_containers(bag: Rc<RefCell<BagSol1>>) -> HashSet<String> {
    let mut result = HashSet::new();
    for c in &bag.borrow().containers {
        result.insert(c.borrow().name.to_owned());
        result.extend(get_containers(Rc::clone(c)))
    }
    result
}

fn get_bag_sol1(name: &str, index: &mut Vec<Rc<RefCell<BagSol1>>>) -> Rc<RefCell<BagSol1>> {
    match index.iter().find(|b| b.borrow().name == name) {
        Some(bag) => Rc::clone(bag),
        None => {
            let bag = Rc::new(RefCell::new(BagSol1 {
                name: name.to_owned(),
                containers: vec![],
            }));

            index.push(Rc::clone(&bag));

            Rc::clone(&bag)
        }
    }
}

#[derive(Debug)]
struct Bag {
    name: String,
    subbags: Vec<(Rc<RefCell<Bag>>, u32)>,
}

#[allow(dead_code)]
pub(crate) fn solution2() -> u32 {
    let input = fs::read_to_string("./input/7.txt").unwrap();

    let bag_regex = Regex::new(r"([a-z ]+) bags contain").unwrap();
    let subs_regex = Regex::new(r"(\d+) ([a-z\s]+) bags*[,.]").unwrap();

    let mut index: Vec<Rc<RefCell<Bag>>> = Vec::new();

    for l in input.lines() {
        let bag_caps = bag_regex.captures(l).unwrap();
        let bag_name = bag_caps.get(1).unwrap().as_str();
        println!("bag name: {}", bag_name);

        let bag = get_bag(bag_name, &mut index);

        for cap in subs_regex.captures_iter(l) {
            let amount: u32 = (&cap[1]).parse().unwrap();
            let sub = &cap[2];
            println!("quantity: {} subbag: {}", amount, sub);

            let subbag = get_bag(sub, &mut index);
            bag.borrow_mut().subbags.push((subbag, amount));
        }
    }

    /*     for bag in &index {
        println!("{:#?}", bag.borrow());
    } */

    let shiny_gold = index
        .iter()
        .find(|b| b.borrow().name == "shiny gold")
        .unwrap();
    get_subbags(Rc::clone(shiny_gold)) - 1
}

fn get_subbags(bag: Rc<RefCell<Bag>>) -> u32 {
    let mut result = 1;
    for (sub, amount) in &bag.borrow().subbags {
        result += get_subbags(Rc::clone(sub)) * amount;
    }
    result
}

fn get_bag(name: &str, index: &mut Vec<Rc<RefCell<Bag>>>) -> Rc<RefCell<Bag>> {
    match index.iter().find(|b| b.borrow().name == name) {
        Some(bag) => Rc::clone(bag),
        None => {
            let bag = Rc::new(RefCell::new(Bag {
                name: name.to_owned(),
                subbags: vec![],
            }));

            index.push(Rc::clone(&bag));

            Rc::clone(&bag)
        }
    }
}
