#![allow(dead_code)]

/*use std::collections::HashMap;
use std::str::Chars;
use advent_of_code::read_lines;

pub fn a() {
    let (mut pattern, rules) = read_input();

    for _ in 0..10 {
        let tmp = apply_rules(&mut pattern.chars(), &rules);
        pattern = tmp;
    }
    println!("{}", calc_result(&mut pattern.chars()));
}

pub fn b() {
    let (mut pattern, rules) = read_input();
    let last_char = pattern.chars().last().unwrap();
    let mut pattern_map = create_pattern_map(&pattern);

    for _ in 0..40 {
        pattern_map = apply_rules_map(pattern_map, &rules);
    }
    println!("{}", calc_result_map(pattern_map.get_map(), last_char));
}

fn calc_result_map(map: HashMap<(char, char), u64>, last_char: char) -> u64 {
    let mut counter = HashMap::new();
    counter.insert(last_char, 1);
    for ((c0, _c1), count) in map {
        let tmp = counter.get(&c0);
        if tmp.is_some() {
            counter.insert(c0, tmp.unwrap()+count);
        } else {
            counter.insert(c0, count);
        }
    }

    println!("{:?}", &counter);

    let mut min = ('x', u64::MAX);
    let mut max = ('x', u64::MIN);
    for curr in counter {
        if curr.1 < min.1 {
            min = curr;
        }
        if curr.1 > max.1 {
            max = curr;
        }
    }


    max.1 - min.1
}

fn apply_rules_map(pattern: CountingSet, rules: &HashMap<(char, char), char>) -> CountingSet {
    let mut res = CountingSet::new();

    for ((c0, c1), count) in pattern.get_map() {
        match rules.get(&(c0, c1)) {
            None => res.add((c0, c1), count),
            Some(x) => {
                res.add((c0, *x), count);
                res.add((*x, c1), count);
            }
        }
    }

    res
}

fn create_pattern_map(str: &str) -> CountingSet {
    let mut res = CountingSet::new();
    let mut chars = str.chars();
    let mut prev = chars.next().unwrap();
    loop {
        let curr = match chars.next() {
            None => break,
            Some(x) => x,
        };
        res.add((prev, curr), 1);
        prev = curr;
    }
    res
}

fn calc_result(chars: &mut Chars) -> u64 {
    let mut counter = HashMap::new();
    for char in chars {
        let tmp = counter.get(&char);
        if tmp.is_some() {
            counter.insert(char, tmp.unwrap()+1);
        } else {
            counter.insert(char, 1u64);
        }
    }

    let mut min = ('x', u64::MAX);
    let mut max = ('x', u64::MIN);
    for curr in counter {
        if curr.1 < min.1 {
            min = curr;
        }
        if curr.1 > max.1 {
            max = curr;
        }
    }


    max.1 - min.1
}

fn apply_rules(pattern: &mut Chars, rules: &HashMap<(char, char), char>) -> String {
    let mut res = vec![];
    let mut prev = pattern.next().unwrap();
    res.push(prev);
    loop {
        let current = match pattern.next() {
            None => break,
            Some(c) => c,
        };
        let rule = rules.get(&(prev, current));
        if rule.is_some() {
            res.push(*rule.unwrap());
        }
        res.push(current);
        prev = current;
    }

    String::from_iter(res)
}

fn read_input() -> (String, HashMap<(char, char), char>) {
    let input = read_lines();

    let pattern = String::from(&input[0]);
    let input = &input[2..];

    let mut rules = HashMap::new();
    for line in input {
        let line = line.trim();
        if line.len() == 0 { continue }
        let mut split = line.split(" -> ");
        let mut tmp = match split.next() {
            None => continue,
            Some(x) => x.chars(),
        };
        let first = match tmp.next() {
            None => continue,
            Some(x) => x,
        };
        let second = match tmp.next() {
            None => continue,
            Some(x) => x,
        };
        let mut tmp = match split.next() {
            None => continue,
            Some(x) => x.chars(),
        };
        let third = match tmp.next() {
            None => continue,
            Some(x) => x,
        };
        rules.insert((first, second), third);
    }

    (pattern, rules)
}

#[derive(Debug)]
pub struct CountingSet {
    set: HashMap<(char, char), u64>,
}

impl CountingSet {
    pub fn add(&mut self, key: (char, char), count: u64) {
        let tmp = self.set.get(&key);
        if tmp.is_some() {
            self.set.insert(key, tmp.unwrap()+count);
        } else {
            self.set.insert(key, count);
        }
    }

    pub fn new() -> Self {
        CountingSet{set: HashMap::new()}
    }

    pub fn get_map(self) -> HashMap<(char, char), u64> {
        self.set
    }
}
*/