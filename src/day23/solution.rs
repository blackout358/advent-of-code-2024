use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    let content = fs::read_to_string("src/day23/data.txt").expect("Error opening file");

    let mut connections_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in content.split("\n") {
        let parts: Vec<&str> = line.split("-").collect();
        // println!("{:?}", parts);

        // if parts[0].contains("t") | parts[1].contains("t") {
        connections_map
            .entry(parts[0])
            .or_insert(HashSet::new())
            .insert(parts[1]);

        connections_map
            .entry(parts[1])
            .or_insert(HashSet::new())
            .insert(parts[0]);
        // }
    }

    println!("{:#?}", connections_map);

    // println!("{:?}", ad);
    let mut result_vec: HashSet<Vec<&&str>> = HashSet::new();
    for (key, hashset) in connections_map.iter() {
        for val in hashset {
            if let Some(second_set) = connections_map.get(val) {
                let intersections = hashset.intersection(second_set);
                // println!("{:?}", intersections);
                for intersect in intersections {
                    let mut res_vec = vec![key, val, intersect];
                    res_vec.sort();
                    // | val.chars().nth(0).unwrap() == 't' | intersect.chars().nth(0) == 't'
                    if key.chars().nth(0).unwrap() == 't'
                        || key.chars().nth(0).unwrap() == 't'
                        || intersect.chars().nth(0).unwrap() == 't'
                    {
                        result_vec.insert(res_vec);
                    }

                    // println!("{:?}", res_vec)
                }
            }
        }
    }

    println!("{:?} {}", result_vec, result_vec.len())
}
