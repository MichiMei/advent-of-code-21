#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code::read_int_list();

    let mut count = 0;
    for index in 0..(input.len()-1) {
        if input[index] < input[index+1] {
            count+=1;
        }
    }
    println!("count: {}", count);
}

pub fn b() {
    let input = advent_of_code::read_int_list();

    let mut count = 0;
    let mut index = 3;
    let mut window = input[0]+input[1]+input[2];
    while index < input.len() {
        let tmp = window + input[index] - input[index-3];
        if tmp > window {
            count += 1;
        }
        index+=1;
        window = tmp;
    }
    println!("count: {}", count);
}
