#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let input = read_lines();
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input {
        let (command, param) = match parse_line(line){
            Ok(x) => x,
            Err(_) => continue,
        };
        match command.as_str() {
            "forward" => horizontal += param,
            "down" => depth += param,
            "up" => depth -= param,
            _ => {
                println!("unknown command");
                continue;
            }
        }
    }
    println!("horizontal position {}", horizontal);
    println!("depth position {}", depth);
}

pub fn b() {
    let input = read_lines();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input {
        let (command, param) = match parse_line(line){
            Ok(x) => x,
            Err(_) => continue,
        };
        match command.as_str() {
            "forward" => {
                horizontal += param;
                depth += aim*param;
            }
            "down" => aim += param,
            "up" => aim -= param,
            _ => {
                println!("unknown command");
                continue;
            }
        }
    }
    println!("horizontal position {}", horizontal);
    println!("depth position {}", depth);
}

fn parse_line(line: String) -> Result<(String, i32), ()> {
    let mut split = line.split(" ");
    let command = match split.next() {
        None => {
            println!("no command");
            return Err(());
        }
        Some(x) => x
    };
    let param = match split.next() {
        None => {
            println!("no param");
            return Err(());
        }
        Some(x) => match x.parse::<i32>() {
            Ok(y) => y,
            Err(_) => {
                println!("parsing failed");
                return Err(());
            }
        }
    };
    Ok((String::from(command), param))
}