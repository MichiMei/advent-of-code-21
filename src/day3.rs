#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let input = read_lines();
    let counts = count_all_ones(&input);
    let total = input.len();
    let th = total/2;

    let mut gamma = 0;
    let mut epsilon = 0;

    for count in counts {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if count > th {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("gamma {}", gamma);
    println!("epsilon {}", epsilon);
    println!("total {}", gamma*epsilon);
}

pub fn b() {
    let mut oxygens = read_lines();
    let mut co2s = oxygens.clone();

    let mut index = 0;
    while oxygens.len() > 1 {
        let count = count_ones(&oxygens, index);
        let th = (oxygens.len()+1)/2;
        if count >= th {
            // retain all 1
            oxygens.retain(|x| { x.as_bytes()[index] as char == '1' });
        } else {
            // retain all 0
            oxygens.retain(|x| { x.as_bytes()[index] as char == '0' });
        }
        index += 1;
    }

    let mut index = 0;
    while co2s.len() > 1 {
        let count = count_ones(&co2s, index);
        let th = (co2s.len()+1)/2;
        if count >= th {
            // retain all 0
            co2s.retain(|x| { x.as_bytes()[index] as char == '0' });
        } else {
            // retain all 1
            co2s.retain(|x| { x.as_bytes()[index] as char == '1' });
        }
        index += 1;
    }

    println!("oxygen {}", oxygens[0]);
    println!("co2 {}", co2s[0]);

    let i1 = i32::from_str_radix(oxygens[0].as_str(), 2).unwrap();
    let i2 = i32::from_str_radix(co2s[0].as_str(), 2).unwrap();

    println!("total {}", i1*i2);
}

fn count_all_ones(input: &Vec<String>) -> Vec<usize> {
    let mut res = vec![];
    res.resize(input[0].len(), 0);
    for line in input {
        let mut char_iter = line.chars();
        for index in 0..res.len() {
            match char_iter.next() {
                Some('1') => res[index] += 1,
                Some('0') => {}
                None => {
                    println!("wrong length");
                    panic!();
                }
                _ => {
                    println!("unsupported char");
                    panic!();
                }
            }
        }
    }
    res
}

fn count_ones(input: &Vec<String>, pos: usize) -> usize {
    let mut res = 0;
    for line in input {
        match line.as_bytes()[pos] as char {
            '1' => res += 1,
            '0' => {}
            _ => {
                println!("unsupported char");
                panic!();
            }
        }
    }
    res
}