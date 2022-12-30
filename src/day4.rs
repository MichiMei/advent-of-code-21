#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let input = read_lines();
    let (commands, mut bingos) = parse_input(input);

    for number in commands {

        for bingo in bingos.iter_mut() {
            if bingo.remove(number).is_some() {
                let res = bingo.get_alive() * number as u32;
                println!("found {}", res);
                return;
            }
        }

    }
}

pub fn b() {
    let input = read_lines();
    let (commands, mut bingos) = parse_input(input);

    let mut count = bingos.len();
    for number in commands {

        for bingo in bingos.iter_mut() {
            if bingo.is_alive() {
                if bingo.remove(number).is_some() {
                    count -= 1;
                    if count == 0 {
                        let res = bingo.get_alive() * number as u32;
                        println!("found {}", res);
                        return;
                    }
                }
            }
        }

    }
}

fn parse_input(input: Vec<String>) -> (Vec<u8>, Vec<Bingo>) {
    let c = parse_line(input[0].as_str(), ",");

    let mut b = vec![];
    let mut index = 2;
    while index+4 < input.len() {
        b.push(parse_bingo(&input[index..(index+5)]));
        index += 6;
    }

    (c, b)
}

fn parse_line(line: &str, pat: &str) -> Vec<u8> {
    let mut res = vec![];

    let split = line.split(pat);
    for word in split {
        let word = word.trim();
        if word.len() == 0 {
            continue;
        }
        res.push(match word.parse() {
           Ok(x) => x,
            Err(x) => panic!("{}", x),
        });
    }

    res
}

fn parse_bingo(lines: &[String]) -> Bingo {
    let mut data = vec![];

    for line in lines {
        let ints = parse_line(line, " ");
        let mut data_line = vec![];
        for elem in ints {
            data_line.push(Some(elem));
        }
        data.push(data_line);
    }

    Bingo {
        data,
        alive: true,
    }
}

#[derive(Debug)]
struct Bingo {
    data: Vec<Vec<Option<u8>>>,
    alive: bool,
}

impl Bingo {
    fn remove(&mut self, elem: u8) -> Option<()> {
        let mut pos = None;
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if self.data[i][j] == Some(elem) {
                    self.data[i][j] = None;
                    pos = Some((i, j));
                    break;
                }
            }
        }

        if pos == None {
            return None;
        }

        let i = pos.unwrap().0;
        let j = pos.unwrap().1;
        let mut i_b = true;
        let mut j_b = true;
        for index in 0..self.data.len() {
            if self.data[i][index].is_some() {
                i_b = false;
            }
            if self.data[index][j].is_some() {
                j_b = false;
            }
        }
        if i_b == true || j_b == true {
            self.alive = false;
            return Some(());
        }
        None
    }

    fn get_alive(&self) -> u32 {
        let mut sum = 0;

        for row in self.data.iter() {
            for elem in row {
                if elem.is_some() {
                    sum += elem.unwrap() as u32;
                }
            }
        }

        sum
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}