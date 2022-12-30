#![allow(dead_code)]

use std::collections::HashSet;
use advent_of_code::read_lines;

pub fn a() {
    let map = read_map();

    let mut risk = 0;

    for (r_index, row) in map.iter().enumerate() {
        for (c_index, elem) in row.iter().enumerate() {
            // check left
            if r_index > 0 && map[r_index-1][c_index] <= *elem {
                continue
            }
            // check right
            if r_index+1 < map.len() && map[r_index+1][c_index] <= *elem {
                continue
            }
            // check up
            if c_index > 0 && map[r_index][c_index-1] <= *elem {
                continue
            }
            // check right
            if c_index+1 < map[r_index].len() && map[r_index][c_index+1] <= *elem {
                continue
            }
            // low point
            println!("[{}, {}] {}", r_index, c_index, elem+1);
            risk += 1+(*elem as u64);
        }
    }

    println!("risk: {}", risk);
}

pub fn b() {
    let map = read_map();

    let mut basins = vec![];

    for (r_index, row) in map.iter().enumerate() {
        for (c_index, elem) in row.iter().enumerate() {
            // check left
            if r_index > 0 && map[r_index-1][c_index] <= *elem {
                continue
            }
            // check right
            if r_index+1 < map.len() && map[r_index+1][c_index] <= *elem {
                continue
            }
            // check up
            if c_index > 0 && map[r_index][c_index-1] <= *elem {
                continue
            }
            // check right
            if c_index+1 < map[r_index].len() && map[r_index][c_index+1] <= *elem {
                continue
            }
            // low point
            basins.push(calc_basin(&map, r_index, c_index))
        }
    }

    basins.sort();

    let res = basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap();
    println!("{}", res);
}

fn calc_basin(map: &Vec<Vec<u8>>, r: usize, c: usize) -> u64 {
    let mut used = HashSet::new();
    used.insert((r,c));
    let mut queue = vec![];
    queue.push((r,c));

    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();
        if i > 0 && map[i-1][j] != 9 {
            if !used.contains(&(i-1,j)) {
                used.insert((i-1,j));
                queue.push((i-1,j));
            }
        }
        // check right
        if i+1 < map.len() && map[i+1][j] != 9 {
            if !used.contains(&(i+1,j)) {
                used.insert((i+1,j));
                queue.push((i+1,j));
            }
        }
        // check up
        if j > 0 && map[i][j-1] != 9 {
            if !used.contains(&(i,j-1)) {
                used.insert((i,j-1));
                queue.push((i,j-1));
            }
        }
        // check right
        if j+1 < map[i].len() && map[i][j+1] != 9 {
            if !used.contains(&(i,j+1)) {
                used.insert((i,j+1));
                queue.push((i,j+1));
            }
        }
    }

    used.len() as u64
}

fn read_map() -> Vec<Vec<u8>> {
    let input = read_lines();
    let mut res = vec![];
    res.resize(input.len(), vec![]);

    for (index, line) in input.into_iter().enumerate() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let tmp = &mut res[index];
        tmp.reserve(line.len());
        for char in line.chars() {
            if !char.is_numeric() {
                panic!()
            }
            let digit = match char.to_digit(10) {
                None => panic!(),
                Some(x) => x as u8,
            };
            tmp.push(digit);
        }
    }
    res
}