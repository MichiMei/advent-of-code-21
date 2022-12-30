#![allow(dead_code)]

use std::io::stdin;

pub fn a() {
    let input = get_input();
    let max = input.iter().max().unwrap();
    let mut numbers = vec![0u32; (max+1) as usize];

    for elem in input.iter() {
        numbers[*elem as usize] += 1;
    }

    let mut fuel: u32 = input.iter().sum();
    let mut smaller_count = numbers[0];
    let mut bigger_count = (input.len() as u32) - numbers[0];

    for pos in 1..numbers.len() {
        let tmp = fuel + smaller_count - bigger_count;
        bigger_count -= numbers[pos];
        smaller_count += numbers[pos];
        if tmp > fuel {
            break
        }
        fuel = tmp;
    }
    println!("{}", fuel);
}

pub fn b() {
    let input = get_input();
    let max = input.iter().max().unwrap();
    let mut numbers = vec![0u32; (max+1) as usize];
    for elem in input.iter() {
        numbers[*elem as usize] += 1;
    }

    let mut fuel = calc_dist(&numbers, 0);
    for pos in 1..numbers.len() {
        let tmp = calc_dist(&numbers, pos);
        if tmp > fuel {
            break
        }
        fuel = tmp;
    }

    println!("{}", fuel);
}

fn calc_dist(numbers: &Vec<u32>, pos: usize) -> u32 {
    let mut fuel = 0;
    let mut sum = 0;
    for curr_pos in pos..numbers.len() {
        sum += (curr_pos-pos) as u32;
        fuel += sum*numbers[curr_pos];
    }
    sum = 0;
    for curr_pos in (0..=pos).rev() {
        sum += (pos-curr_pos) as u32;
        fuel += sum*numbers[curr_pos];
    }

    fuel
}

fn get_input() -> Vec<u32> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("reading failed");

    let split = input.trim().split(",");
    let res = split.map(|x| x.trim().parse().unwrap()).collect();

    res
}