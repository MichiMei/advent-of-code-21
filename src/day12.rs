#![allow(dead_code)]

use advent_of_code::read_lines;

pub fn a() {
    let graph = read_graph();
    let start = graph.get_node_id("start").unwrap();
    let mut visited = vec![];
    let mut path = vec![];
    let res = find_paths(&graph, start, &mut visited, &mut path);
    println!("{}", res);
}

pub fn b() {
    let graph = read_graph();
    let start = graph.get_node_id("start").unwrap();
    let mut visited = vec![];
    let mut path = vec![];
    let res = find_paths_twice(&graph, start, &mut visited, &mut path, true);
    println!("{}", res);
}

fn find_paths(graph: &Graph, node: usize, visited: &mut Vec<usize>, path: &mut Vec<String>) -> u32 {
    //println!("node {}", graph.get_name(node));
    let name = graph.get_name(node);
    if name.eq("end") {
        path.push(String::from(name));
        //println!("found {:?}", path);
        path.pop();
        return 1;
    }
    if name.chars().next().unwrap().is_lowercase() {
        visited.push(node);
    }
    path.push(String::from(name));
    let neighbors = graph.get_neighbors(node);
    let mut res = 0;
    for neighbor in neighbors {
        if !visited.contains(neighbor) {
            res += find_paths(graph, *neighbor, visited, path);
        }
    }
    if name.chars().next().unwrap().is_lowercase() {
        visited.pop();
    }
    path.pop();
    res
}

fn find_paths_twice(graph: &Graph, node: usize, visited: &mut Vec<usize>, path: &mut Vec<String>, twice: bool) -> u32 {
    //println!("path {:?} node {}", path, graph.get_name(node));
    let name = graph.get_name(node);
    if name.eq("end") {
        path.push(String::from(name));
        //println!("found {:?}", path);
        path.pop();
        return 1;
    }
    if name.chars().next().unwrap().is_lowercase() {
        visited.push(node);
    }
    path.push(String::from(name));
    let neighbors = graph.get_neighbors(node);
    let mut res = 0;
    for neighbor in neighbors {
        if !visited.contains(neighbor) {
            res += find_paths_twice(graph, *neighbor, visited, path, twice);
        }
        if visited.contains(neighbor) && twice && !graph.get_name(*neighbor).eq("start"){
            res += find_paths_twice(graph, *neighbor, visited, path, false);
        }
    }
    if name.chars().next().unwrap().is_lowercase() {
        visited.pop();
    }
    path.pop();
    res
}

fn read_graph() -> Graph {
    let input = read_lines();
    let mut graph = Graph {
        edges: vec![],
        names: vec![],
    };
    for line in input {
        let line = line.trim();
        if line.len() == 0 {
            continue
        }
        let edge = parse_line(line);
        graph.insert_edge_by_name(&edge.0, &edge.1);
    }
    graph
}

fn parse_line(str: &str) -> (String, String) {
    let mut split = str.trim().split("-");
    let node0 = split.next().unwrap().trim();
    let node1 = split.next().unwrap().trim();
    (String::from(node0), String::from(node1))
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<usize>>,
    names: Vec<String>,
}

impl Graph {
    pub fn get_node_id(&self, node: &str) -> Option<usize> {
        for elem in self.names.iter().enumerate() {
            if elem.1.eq(node) {
                return Some(elem.0)
            }
        }
        None
    }

    pub fn insert_edge(&mut self, u: usize, v: usize) -> Result<(), ()> {
        if self.names.len() <= u || self.names.len() <= v {
            return Err(());
        }

        self.edges[u].push(v);
        self.edges[v].push(u);

        Ok(())
    }

    pub fn insert_edge_by_name(&mut self, u: &str, v: &str) {
        let u_index = self.insert_node(u);
        let v_index = self.insert_node(v);
        self.insert_edge(u_index, v_index).unwrap();
    }

    pub fn insert_node(&mut self, name: &str) -> usize {
        let tmp = self.get_node_id(name);
        if tmp.is_none() {
            self.names.push(String::from(name));
            self.edges.push(vec![]);
            return self.names.len()-1;
        }
        tmp.unwrap()
    }

    pub fn get_name(&self, node: usize) -> &str {
        &self.names[node]
    }

    pub fn get_neighbors(&self, node: usize) -> &[usize] {
        &self.edges[node]
    }
}