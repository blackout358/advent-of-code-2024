use std::collections::VecDeque;
use std::{cmp, fs, usize};
const SIZE: usize = 71;

pub struct Move {
    score: i32,
    y: i32,
    x: i32,
}

pub fn solve() {
    let content = fs::read_to_string("src/day18/data.txt").expect("Error opening file");
    // println!("{content}");

    let mut map = [['.'; SIZE]; SIZE];

    map[SIZE - 1][SIZE - 1] = 'f';

    let directions: [[i32; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

    let mut moves: VecDeque<Move> = VecDeque::new();

    let _ = content.split('\n').take(1025).for_each(|asd| {
        let corr_loc: Vec<usize> = asd
            .split(',')
            .map(|pos| pos.parse::<usize>().unwrap())
            .collect();

        map[corr_loc[1]][corr_loc[0]] = '#';
        println!("{}", asd)
    });
    moves.push_back(Move {
        score: 0,
        y: 0,
        x: 0,
    });
    let mut shortest: i32 = i32::MAX;

    while !moves.is_empty() {
        let curr_move = moves.pop_front().unwrap();
        map[curr_move.y as usize][curr_move.x as usize] = 'O';
        for direction in directions {
            let new_y = curr_move.y + direction[0];
            let new_x = curr_move.x + direction[1];

            if new_y == SIZE as i32 - 1 && new_x == SIZE as i32 - 1 {
                shortest = cmp::min(curr_move.score + 1, shortest);
                continue;
            }

            if new_y >= 0
                && new_y < SIZE as i32
                && new_x >= 0
                && new_x < SIZE as i32
                && map[new_y as usize][new_x as usize] == '.'
            {
                moves.push_back(Move {
                    score: curr_move.score + 1,
                    y: new_y,
                    x: new_x,
                });
                map[new_y as usize][new_x as usize] = '*';
            }
        }
    }
}

pub fn part_two() {
    let content = fs::read_to_string("src/day18/data.txt").expect("Error opening file");

    let mut map: [[char; 71]; 71] = [['.'; SIZE]; SIZE];
    let _ = content.split('\n').take(1024).for_each(|asd| {
        let corr_loc: Vec<usize> = asd
            .split(',')
            .map(|pos| pos.parse::<usize>().unwrap())
            .collect();

        map[corr_loc[1]][corr_loc[0]] = '#';
    });
    map[SIZE - 1][SIZE - 1] = 'f';
    for next_move in content.split('\n').skip(1024) {
        let next_blockade = next_move
            .split(",")
            .map(|asd| asd.parse().unwrap())
            .collect::<Vec<usize>>();

        map[next_blockade[1]][next_blockade[0]] = '#';
        let mut local_map = map.clone();

        if dfs_helper(&mut local_map) {
            println!("{}", next_move);
            break;
        }
    }

    fn dfs_helper(map: &mut [[char; 71]; 71]) -> bool {
        let directions: [[i32; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];
        dfs(map, &directions)
    }

    fn dfs(map: &mut [[char; 71]; 71], directions: &[[i32; 2]; 4]) -> bool {
        let mut stack = vec![(0, 0)];

        while let Some((y, x)) = stack.pop() {
            if map[y as usize][x as usize] == 'f' {
                return false;
            }

            map[y as usize][x as usize] = 'O';
            for direction in directions {
                let new_y = y + direction[0];
                let new_x = x + direction[1];

                if new_y >= 0
                    && new_x >= 0
                    && new_y < SIZE as i32
                    && new_x < SIZE as i32
                    && (map[new_y as usize][new_x as usize] == '.'
                        || map[new_y as usize][new_x as usize] == 'f')
                {
                    stack.push((new_y, new_x));
                }
            }
        }

        true
    }

    fn _print_map(map: &[[char; SIZE]; SIZE]) {
        for row in map.iter() {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
}
