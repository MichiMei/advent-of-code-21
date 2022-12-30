#![allow(dead_code)]

use std::io::stdin;

pub fn a() {
    let mut population = get_input();
    let mut days = 256;

    while days > 0 {
        println!("{} days remaining", days);
        let mut count = 0;
        for elem in population.iter_mut() {
            if *elem == 0u8 {
                *elem = 6u8;
                count += 1;
            } else {
                *elem -= 1;
            }
        }
        population.resize(population.len()+count, 8u8);
        days -= 1;
    }
    println!("{}", population.len());
}

pub fn b() {
    let population = get_input();
    let mut days = 256;

    let mut ring_buff = vec![0u64; 7];
    println!("len {}", ring_buff.len());
    let mut pointer = 0;
    for elem in population {
        ring_buff[elem as usize] += 1;
    }
    let mut day8 = 0u64;
    let mut day7 = 0u64;

    while days > 0 {
        let tmp = ring_buff[pointer];
        ring_buff[pointer] += day7;
        day7 = day8;
        day8 = tmp;
        pointer = (pointer+1)%7;
        days -= 1;
    }
    let mut sum = day7 + day8;
    for elem in ring_buff {
        sum += elem;
    }
    println!("{}", sum);
}

fn get_input() -> Vec<u8> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("reading failed");

    let split = input.trim().split(",");
    let res = split.map(|x| x.trim().parse().unwrap()).collect();

    res
}