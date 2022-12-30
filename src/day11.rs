#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let mut grid = read_grid();

    let step_count = 100;

    let mut flash_count = 0;

    for step in 1..=step_count {
        inc_step(&mut grid);
        flash_count += flash_step(&mut grid, step);
        reset_step(&mut grid);
    }

    println!("{}", flash_count);
}

pub fn b() {
    let mut grid = read_grid();

    let mut step_count = 0;

    loop {
        step_count += 1;
        inc_step(&mut grid);
        let tmp = flash_step(&mut grid, step_count);
        if tmp == 100 {
            break;
        }
        reset_step(&mut grid);
    }

    println!("{}", step_count);
}

fn inc_step(grid: &mut Vec<Vec<(u8, u32)>>) {
    for row in grid.iter_mut() {
        for elem in row.iter_mut() {
            elem.0 += 1;
        }
    }
}

fn flash_step(grid: &mut Vec<Vec<(u8, u32)>>, step: u32) -> u32 {
    let mut res = 0;
    for r_index in 0..grid.len() {
        for c_index in 0..grid[r_index].len() {
            if grid[r_index][c_index].1 < step && grid[r_index][c_index].0 > 9 {
                grid[r_index][c_index].1 = step;
                res += 1;
                if r_index > 0 && c_index > 0 {
                    grid[r_index-1][c_index-1].0 += 1;
                }
                if r_index+1 < grid.len() && c_index > 0 {
                    grid[r_index+1][c_index-1].0 += 1;
                }
                if r_index > 0 && c_index+1 < grid[r_index].len() {
                    grid[r_index-1][c_index+1].0 += 1;
                }
                if r_index+1 < grid.len() && c_index+1 < grid[r_index].len() {
                    grid[r_index+1][c_index+1].0 += 1;
                }
                if r_index > 0 {
                    grid[r_index-1][c_index].0 += 1;
                }
                if c_index > 0 {
                    grid[r_index][c_index-1].0 += 1;
                }
                if r_index+1 < grid.len() {
                    grid[r_index+1][c_index].0 += 1;
                }
                if c_index+1 < grid[r_index].len() {
                    grid[r_index][c_index+1].0 += 1;
                }
            }
        }
    }

    if res > 0 {
        res += flash_step(grid, step);
    }
    res
}

fn reset_step(grid: &mut Vec<Vec<(u8, u32)>>) {
    for row in grid.iter_mut() {
        for elem in row.iter_mut() {
            if elem.0 > 9 {
                elem.0 = 0;
            }
        }
    }
}

fn read_grid() -> Vec<Vec<(u8, u32)>>{
    let mut input = read_lines().into_iter();
    let mut res = vec![vec![]; 10];

    for tmp in res.iter_mut() {
        tmp.reserve(10);
        let line = input.next().unwrap();

        for char in line.chars() {
            if !char.is_numeric() {
                panic!()
            }
            let digit = match char.to_digit(10) {
                None => panic!(),
                Some(x) => x as u8,
            };
            tmp.push((digit, 0));
            if tmp.len() == 10 {
                break;
            }
        }

    }

    res
}