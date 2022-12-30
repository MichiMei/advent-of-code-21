#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let input = read_lines();
    let lines = parse_lines(input);
    println!("{}", find_easy_digits(&lines));
}

pub fn b() {
    let input = read_lines();
    let lines = parse_lines(input);
    let mut sum = 0;
    for elem in lines {
        let res = match_line(elem.0, elem.1);
        sum += res;
    }
    println!("{}", sum);
}

fn match_line(mut patterns: Vec<String>, digits: Vec<String>) -> u32 {
    let mut matched = vec![String::new(); 10];
    matched[1] = find_one(&mut patterns);
    matched[4] = find_four(&mut patterns);
    matched[7] = find_seven(&mut patterns);
    matched[8] = find_eight(&mut patterns);
    matched[3] = find_three(&mut patterns, &matched[1]);
    matched[6] = find_six(&mut patterns, &matched[1]);

    let segment_c = get_segment_c(&matched[6]);

    matched[2] = find_two(&mut patterns, segment_c);
    matched[5] = find_five(&mut patterns);
    matched[9] = find_nine(&mut patterns, &matched[5]);
    matched[0] = find_zero(&mut patterns);

    let mut number = 0;
    for digit in digits {
        number *= 10;
        let tmp = match_digit(&digit, &matched);
        number += tmp;
    }

    number
}

fn match_digit(digit: &str, patterns: &Vec<String>) -> u32{
    for index in 0..patterns.len() {
        if get_ordered(digit).eq(&patterns[index]) {
            return index as u32;
        }
    }
    panic!()
}

fn find_one(patterns: &mut Vec<String>) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() == 2 {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_four(patterns: &mut Vec<String>) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() == 4 {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_seven(patterns: &mut Vec<String>) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() == 3 {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_eight(patterns: &mut Vec<String>) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() == 7 {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_three(patterns: &mut Vec<String>, one: &str) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() != 5 {
            continue;
        }
        if contains(&patterns[index], &one) {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_six(patterns: &mut Vec<String>, one: &str) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() != 6 {
            continue;
        }
        if !contains(&patterns[index], &one) {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn get_segment_c(six: &str) -> char {
    let pattern = "abcdefg";
    for char in pattern.chars() {
        if !six.contains(char) {
            return char;
        }
    }
    panic!()
}

fn find_two(patterns: &mut Vec<String>, segment_c: char) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() != 5 {
            continue;
        }
        if patterns[index].contains(segment_c) {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_five(patterns: &mut Vec<String>) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() != 5 {
            continue;
        }
        pattern = get_ordered(&patterns[index]);
        patterns.remove(index);
        break;
    }
    pattern
}

fn find_nine(patterns: &mut Vec<String>, five: &str) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() != 6 {
            continue;
        }
        if contains(&patterns[index], &five) {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn find_zero(patterns: &mut Vec<String>) -> String {
    let mut pattern = String::new();
    for index in 0..patterns.len() {
        if patterns[index].len() == 6 {
            pattern = get_ordered(&patterns[index]);
            patterns.remove(index);
            break;
        }
    }
    pattern
}

fn get_ordered(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

fn contains(input: &str, pat: &str) -> bool {
    for char in pat.chars() {
        if !input.contains(char) {
            return false;
        }
    }
    true
}

fn find_easy_digits(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let mut res = 0;
    for line in input.iter() {
        for digit in line.1.iter() {
            if digit.len() == 2 ||
                digit.len() == 3 ||
                digit.len() == 4 ||
                digit.len() == 7 {
                res += 1;
            }
        }
    }
    res
}

fn parse_lines(input: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {

    let mut res = vec![];
    for line in input {
        match parse_line(&line) {
            None => continue,
            Some(x) => res.push(x),
        };
    }
    res
}

fn parse_line(line: &str) -> Option<(Vec<String>, Vec<String>)> {
    let mut split = line.split("|");

    let patterns = match parse_region(split.next()) {
        None => return None,
        Some(x) => x,
    };

    let digits = match parse_region(split.next()) {
        None => return None,
        Some(x) => x,
    };

    Some((patterns, digits))
}

fn parse_region(input: Option<&str>) -> Option<Vec<String>> {
    let pattern_split = match input {
        None => return None,
        Some(x) => {
            let tmp = x.trim();
            if tmp.len() == 0 {
                return None;
            }
            tmp
        }
    };
    let mut res = vec![];
    let pattern_split = pattern_split.split(" ");
    for elem in pattern_split {
        let tmp = String::from(elem.trim());
        res.push(tmp);
    }
    Some(res)
}