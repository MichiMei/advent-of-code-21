#![allow(dead_code)]


use std::collections::HashSet;
use advent_of_code::read_lines;

pub fn a() {
    let (points, commands) = get_input();
    println!("before {}", points.len());
    //visualize(&points);
    let mut tmp = HashSet::new();
    let command = commands.first().unwrap();
    for point in points {
        tmp.insert(point.fold(command.0, command.1));
    }
    println!("after {}", tmp.len());
    //visualize(&tmp);
}

pub fn b() {
    let (mut points, commands) = get_input();
    println!("before {}", points.len());

    for command in commands {
        let mut tmp = HashSet::new();
        for point in points {
            tmp.insert(point.fold(command.0, command.1));
        }
        points = tmp;
    }

    println!("after {}", points.len());
    visualize(&points);
}

fn visualize(points: &HashSet<Point>) {
    let mut x_max = 0;
    let mut y_max = 0;
    for point in points.iter() {
        if point.x > x_max { x_max = point.x; }
        if point.y > y_max { y_max = point.y; }
    }

    let mut array = vec![vec![]; y_max+1];
    for row in array.iter_mut() {
        row.resize(x_max+1, '.');
    }

    for point in points.iter() {
        array[point.y][point.x] = '#';
    }

    for row in array {
        println!("{}", String::from_iter(row));
    }
}

fn get_input() -> (HashSet<Point>, Vec<(Direction, usize)>) {
    let input = read_lines();
    let mut empty_line = 0;
    for (index, line) in input.iter().enumerate() {
        if line.trim().len() == 0 {
            empty_line = index;
            break;
        }
    }
    if empty_line == 0 {panic!()}

    let mut points = HashSet::new();
    for index in 0..empty_line {
        let line = input[index].trim();
        if line.len() == 0 {panic!()}
        let mut split = line.split(",");
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        points.insert(Point{x,y});
    }
    let mut commands = vec![];
    for index in empty_line+1..input.len() {
        let line = input[index].trim();
        if line.len() == 0 {continue}
        commands.push(parse_command(line))
    }

    (points, commands)
}

fn parse_command(str: &str) -> (Direction, usize) {
    let mut split = str.split("=");
    let dir = match split.next() {
        Some("fold along x") => Direction::Vertical,
        Some("fold along y") => Direction::Horizontal,
        _ => panic!(),
    };
    let pos = split.next().unwrap().parse().unwrap();

    (dir, pos)
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn fold(&self, dir: Direction, pos: usize) -> Point{
        match dir {
            Direction::Horizontal => self.fold_horizontal(pos),
            Direction::Vertical => self.fold_vertical(pos),
        }
    }

    fn fold_vertical(&self, pos: usize) -> Point {
        if self.x > pos {
            let new_x = pos-(self.x-pos);
            if new_x > pos {panic!()}
            return Point{x:new_x, y:self.y}
        }
        self.clone()
    }

    fn fold_horizontal(&self, pos: usize) -> Point {
        if self.y > pos {
            let new_y = pos-(self.y-pos);
            if new_y > pos {panic!()}
            return Point{x:self.x, y:new_y}
        }
        self.clone()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}