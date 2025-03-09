use regex::Regex;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

// use std::collections::hash_map::Entry

pub fn solve() {
    let contents = fs::read_to_string("src/day25/data.txt").expect("Error opening file");

    println!("{}\n ", contents);
    let re_pattern = Regex::new(r"([#.]{5}[#.\n]{36})").unwrap();
    // Filter key and locks

    let mut matches = re_pattern.captures_iter(&contents);

    let mut keys: Vec<Vec<u32>> = Vec::new();
    let mut locks: Vec<Vec<u32>> = Vec::new();

    while let Some(regex_match) = matches.next() {
        let extract = regex_match.get(0).unwrap().as_str();
        let rows: Vec<&str> = extract.split("\n").collect();

        let mut column_values: Vec<u32> = vec![0; 5];
        // println!("{:?}", rows);
        if rows[0].contains("#####") {
            for row in &rows[1..] {
                // println!("asd{}", row);
                for (i, char) in row.char_indices() {
                    if char == '#' {
                        column_values[i] += 1;
                    }
                }
            }
            locks.push(column_values);
        } else {
            for row in &rows[..6] {
                for (i, char) in row.char_indices() {
                    if char == '#' {
                        column_values[i] += 1;
                    }
                }
            }
            keys.push(column_values);
        }
    }

    println!("k{:?}\nl{:?}", keys, locks);

    let mut lock_holes: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();

    for lock in locks {
        let entry_result = match lock_holes.entry(lock[0]) {
            Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            Entry::Vacant(vacant_entry) => vacant_entry.insert(vec![]),
        };

        entry_result.push(lock);
    }

    let mut count: u32 = 0;
    for key in keys {
        let rem_space = 5 - key[0];

        // println!("{}", rem_space)
        for pos_slots in 0..=rem_space {
            if let Some(pos_combos) = lock_holes.get(&pos_slots) {
                for pos_combo in pos_combos {
                    println!("{:?} \n{:?}\n\n", key, pos_combo);
                    let mut i: usize = 0;
                    for part in pos_combo {
                        if key[i] + part > 5 {
                            break;
                        }
                        i += 1;
                    }

                    if i == 5 {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{:?}", count)
}
