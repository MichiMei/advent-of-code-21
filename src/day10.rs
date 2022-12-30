#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let input = read_lines();

    let mut sum = 0;

    for line in input {
        let line = line.trim();
        if line.len() == 0 {
            continue
        }
        let (tmp, _) = check_line(line);
        sum += tmp;
    }

    println!("{}", sum);
}

pub fn b() {
    let input = read_lines();

    let mut scores = vec![];

    for line in input {
        let line = line.trim();
        if line.len() == 0 {
            continue
        }
        let (code, bracket_stack) = check_line(line);
        if code != 0 {
            continue;
        }
        scores.push(autocomplete(bracket_stack));
    }

    scores.sort();

    println!("{}", scores[scores.len()/2]);
}

fn autocomplete(mut bracket_stack: Vec<char>) -> u64 {
    let mut score = 0;

    while !bracket_stack.is_empty() {
        score *= 5;
        match bracket_stack.pop().unwrap() {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => panic!(),
        }
    }

    score
}

fn check_line(line: &str) -> (u32, Vec<char>) {
    let mut bracket_stack = vec![];

    for char in line.chars() {

        match char {
            '(' => bracket_stack.push('('),
            '[' => bracket_stack.push('['),
            '{' => bracket_stack.push('{'),
            '<' => bracket_stack.push('<'),
            ')' => {
                let c = bracket_stack.pop();
                if  c.is_none() || c.unwrap() != '(' {
                    return (3, bracket_stack);
                }
            }
            ']' => {
                let c = bracket_stack.pop();
                if  c.is_none() || c.unwrap() != '[' {
                    return (57, bracket_stack);
                }
            }
            '}' => {
                let c = bracket_stack.pop();
                if  c.is_none() || c.unwrap() != '{' {
                    return (1197, bracket_stack);
                }
            }
            '>' => {
                let c = bracket_stack.pop();
                if  c.is_none() || c.unwrap() != '<' {
                    return (25137, bracket_stack);
                }
            }
            _ => panic!(),
        }
    }

    (0, bracket_stack)
}