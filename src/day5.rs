#![allow(dead_code)]

use advent_of_code::read_lines;
//use crate::day5::Direction::{Down, DownRight, Right, UpRight};

pub fn a() {
    let input = read_lines();
    let lines = parse_lines(&input);
    let max = max(&lines)+1;
    let mut board = vec![];
    board.resize(max as usize, vec![]);
    for row in board.iter_mut() {
        row.resize(max as usize, 0);
    }

    for line in lines {
        if is_horizontal(&line) {
            let (line, direction) = normalize(line);
            match direction {
                Direction::Right => {
                    let i = line.0.x as usize;
                    for j in line.0.y..=line.1.y {
                        board[i][j as usize] += 1;
                    }
                }
                Direction::DownRight => unimplemented!(),
                Direction::UpRight => unimplemented!(),
                Direction::Down => {
                    let j = line.0.y as usize;
                    for i in line.0.x..=line.1.x {
                        board[i as usize][j] += 1;
                    }
                }
            }
        }
    }
    println!("{}", count_bigger(&board));
}

pub fn b() {
    let input = read_lines();
    let lines = parse_lines(&input);
    let max = max(&lines)+1;
    let mut board = vec![];
    board.resize(max as usize, vec![]);
    for row in board.iter_mut() {
        row.resize(max as usize, 0);
    }

    for line in lines {
        let (line, direction) = normalize(line);
        match direction {
            Direction::Right => {
                let i = line.0.x as usize;
                for j in line.0.y..=line.1.y {
                    board[i][j as usize] += 1;
                }
            }
            Direction::DownRight => {
                let mut i = line.0.x as usize;
                let mut j = line.0.y as usize;
                for _ in line.0.x..=line.1.x {
                    board[i][j] += 1;
                    i += 1;
                    j += 1;
                }
            }
            Direction::UpRight => {
                let mut i = line.0.x as usize;
                let mut j = line.0.y as usize;
                for _ in line.0.x..=line.1.x {
                    board[i][j] += 1;
                    i += 1;
                    if j > 0 {
                        j -= 1;
                    }
                }
            }
            Direction::Down => {
                let j = line.0.y as usize;
                for i in line.0.x..=line.1.x {
                    board[i as usize][j] += 1;
                }
            }
        }
    }
    println!("{}", count_bigger(&board));
}

fn parse_lines(input: &[String]) -> Vec<(Point, Point)> {
    let mut res = vec![];
    for str_line in input {
        let str_line = str_line.trim();
        if str_line.len() == 0 {
            continue;
        }
        let mut str_points = str_line.split("->");
        let start = Point::parse_point(str_points.next().unwrap());
        let end = Point::parse_point(str_points.next().unwrap());
        res.push((start, end));
    }
    res
}

fn max(input: &[(Point, Point)]) -> u16 {
    let mut max = 0;
    for (p0, p1) in input {
        if p0.x > max {
            max = p0.x;
        }
        if p0.y > max {
            max = p0.y;
        }
        if p1.x > max {
            max = p1.x;
        }
        if p1.y > max {
            max = p1.y;
        }
    }
    max
}

fn is_horizontal(line: &(Point, Point)) -> bool {
    if line.0.x == line.1.x {
        return true;
    }
    if line.0.y == line.1.y {
        return true;
    }
    false
}

fn normalize(mut line: (Point, Point)) -> ((Point, Point), Direction) {
    if is_horizontal(&line) {
        if line.0.x == line.1.x {
            // Down
            if line.0.y < line.1.y {
                ((line.0, line.1), Direction::Right)
            } else {
                ((line.1, line.0), Direction::Right)
            }
        } else {
            // Right
            if line.0.x < line.1.x {
                ((line.0, line.1), Direction::Down)
            } else {
                ((line.1, line.0), Direction::Down)
            }
        }
    } else {
        // normalize to right
        if line.0.x > line.1.x {
            line = (line.1, line.0);
        }
        if line.0.y < line.1.y {
            ((line.0, line.1), Direction::DownRight)
        } else {
            ((line.0, line.1), Direction::UpRight)
        }
    }
}

fn count_bigger(board: &Vec<Vec<u16>>) -> u32 {
    let mut res = 0;
    for row in board {
        for elem in row {
            if elem > &1 {
                res += 1;
            }
        }
    }
    res
}

pub enum Direction {
    Right,
    DownRight,
    UpRight,
    Down,

}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn parse_point(str: &str) -> Point {
        let str = str.trim();
        let mut split = str.split(",");
        let x = split.next().unwrap().trim().parse().unwrap();
        let y = split.next().unwrap().trim().parse().unwrap();
        Point {
            x,
            y,
        }
    }
}