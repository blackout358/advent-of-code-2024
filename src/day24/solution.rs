use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, VecDeque},
    fs,
};
pub fn solve() {
    let content = fs::read_to_string("src/day24/data.txt").expect("Error opening file");
    // println!("{content}");

    let lines: Vec<&str> = content.split("\n").collect();

    let combos_regex = Regex::new(r"([\w\d]+)\s([\w]+)\s([\w]+)\s->\s([\w\d]+)").unwrap();
    let val_reg = Regex::new(r"([\w\d]+):\s(\d)").unwrap();
    let mut val_map: HashMap<&str, u8> = HashMap::new();

    let mut deq: VecDeque<(&str, &str, &str, &str)> = VecDeque::new();
    let mut max_z = 0;
    for line in lines {
        if let Some(matched) = combos_regex.captures(line) {
            // println!("{:?}", matched.);
            let equations = (
                matched.get(1).unwrap().as_str(),
                matched.get(2).unwrap().as_str(),
                matched.get(3).unwrap().as_str(),
                matched.get(4).unwrap().as_str(),
            );

            if equations.3.contains("z") {
                let number: i32 = equations.3[1..].parse().unwrap();
                max_z = cmp::max(max_z, number);
            }

            deq.push_back(equations);
        } else if let Some(my_match) = val_reg.captures(line) {
            println!(
                "{} {}",
                my_match.get(1).unwrap().as_str(),
                my_match.get(2).unwrap().as_str()
            );

            val_map.insert(
                my_match.get(1).unwrap().as_str(),
                (my_match.get(2).unwrap().as_str()).parse().unwrap(),
            );
        }
    }

    println!("{:?}", deq);
    while !deq.is_empty() {
        let equation = deq.pop_front().unwrap();
        if let (Some(option1), Some(option2)) = (val_map.get(equation.0), val_map.get(equation.2)) {
            let outcome: u8;
            match equation.1 {
                "XOR" => outcome = option1 ^ option2,
                "AND" => outcome = option1 & option2,
                "OR" => outcome = option1 | option2,
                _ => outcome = 0,
            }
            val_map.insert(equation.3, outcome);
        } else {
            deq.push_back(equation);
        }
    }
    let mut final_bits = String::new();
    for i in 0..=max_z {
        let lookup = format!("z{:0>2}", i);
        // println!("{lookup}")
        let map_res = val_map.get(lookup.as_str()).unwrap();

        final_bits.push_str(&map_res.to_string());
    }

    // println!("{}", final_bits.chars().rev().collect::<String>())
    let bits_in_order = final_bits.chars().rev().collect::<String>();
    println!("{}", i128::from_str_radix(&bits_in_order, 2).unwrap())
}
