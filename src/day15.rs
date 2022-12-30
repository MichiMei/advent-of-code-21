#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use advent_of_code::read_lines;

pub fn a() {
    let mut board = get_board();
    dijkstra(&mut board);
    println!("{}", board.last().unwrap().last().unwrap().1.unwrap())
}

pub fn b() {
    let mut board = get_big_board();
    dijkstra(&mut board);
    println!("{}", board.last().unwrap().last().unwrap().1.unwrap())
}

fn print(board: &Vec<Vec<(u8, Option<u64>, bool)>>) {
    for line in board.iter() {
        for elem in line.iter() {
            print!("{}", elem.0);
        }
        println!();
    }
}

fn get_big_board() -> Vec<Vec<(u8, Option<u64>, bool)>> {
    let small_board = get_board();
    let mut res = vec![];
    res.reserve(small_board.len()*5);

    for x_incr in 0..5 {

        for line in small_board.iter() {
            let mut tmp = vec![];
            tmp.reserve(line.len()*5);

            for y_incr in 0..5 {

                for elem in line.iter() {
                    let mut new_0 = elem.0+x_incr+y_incr;
                    if new_0 > 9 {
                        new_0 -= 9;
                    }
                    tmp.push((new_0, None, false));
                }
            }
            res.push(tmp);
        }
    }
    res
}

fn dijkstra(board: &mut Vec<Vec<(u8, Option<u64>, bool)>>) {
    let mut pq = BinaryHeap::new();
    pq.push(HeapElem{x:0, y:0, d:0});

    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        if board[curr.x][curr.y].2 == true {continue}
        board[curr.x][curr.y].1 = Some(curr.d);
        board[curr.x][curr.y].2 = true;
        if curr.x > 0 && board[curr.x-1][curr.y].2 == false {
            pq.push(HeapElem{x:curr.x-1, y:curr.y, d:curr.d+(board[curr.x-1][curr.y].0 as u64)});
        }
        if curr.y > 0 && board[curr.x][curr.y-1].2 == false {
            pq.push(HeapElem{x:curr.x, y:curr.y-1, d:curr.d+(board[curr.x][curr.y-1].0 as u64)});
        }
        if curr.x+1 < board.len() && board[curr.x+1][curr.y].2 == false {
            pq.push(HeapElem{x:curr.x+1, y:curr.y, d:curr.d+(board[curr.x+1][curr.y].0 as u64)});
        }
        if curr.y+1 < board[curr.x].len() && board[curr.x][curr.y+1].2 == false {
            pq.push(HeapElem{x:curr.x, y:curr.y+1, d:curr.d+(board[curr.x][curr.y+1].0 as u64)});
        }
        if curr.x+1 == board.len() && curr.y+1 == board[curr.x].len() {
            break;
        }
    }
}

struct HeapElem {
    x:usize,
    y:usize,
    d:u64,
}

impl Eq for HeapElem {}

impl PartialEq<Self> for HeapElem {
    fn eq(&self, other: &Self) -> bool {
        if self.cmp(other) == Ordering::Equal {
            return true
        }
        false
    }
}

impl PartialOrd<Self> for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d)
    }
}

fn get_board() -> Vec<Vec<(u8, Option<u64>, bool)>> {
    let input = read_lines();
    let mut res = vec![];
    res.reserve(input.len());
    for line in input {
        let line = line.trim();
        if line.len() == 0 { continue }
        let mut tmp = vec![];
        tmp.reserve(line.len());
        for char in line.chars() {
            if !char.is_numeric() {
                panic!()
            }
            let digit = match char.to_digit(10) {
                None => panic!(),
                Some(x) => x as u8,
            };
            tmp.push((digit, None, false));
        }
        res.push(tmp);
    }
    res
}