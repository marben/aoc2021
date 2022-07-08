use std::collections::{HashMap};
use std::io::BufRead;

#[derive(Debug)]
struct Connections {
    conn: HashMap<String, Vec<String>>,
}

impl Connections {
    fn add(&mut self, from: &str, to: &str) {
        match self.conn.get_mut(from) {
            None => {
                self.conn.insert(from.to_string(), vec![to.to_string()]);
            }
            Some(v) => {
                v.push(to.to_string());
            }
        }
    }
    fn new() -> Connections {
        Connections{conn: HashMap::new()}
    }
}

fn parse_input_lines_into_caves(input_lines: &Vec<String>) -> Connections {
    let mut connections = Connections::new();
    for l in input_lines {
        let split: Vec<&str> = l.trim().split("-").collect();
        let c1_str = split[0];
        let c2_str = split[1];
        connections.add(c1_str, c2_str);
        connections.add(c2_str, c1_str);
    }
    return connections;
}

#[derive(Debug, Clone)]
struct Path {
    caves: Vec<String>,
    double_visited: bool, // marks a double visit to small cave (part2)
}

impl Path {
    fn new() -> Path {
        Path { caves: vec![], double_visited: false}
    }
    fn add_cave(&mut self, cave: String) {
        self.caves.push(cave);
    }

    fn contains(&self, cave: &str) -> bool {
        for c in &self.caves {
            if cave == c {
                return true;
            }
        }
        false
    }
}

fn find_paths_part_1(conns: &Connections) -> Vec<Path> {
    let mut paths = vec![];
    let mut initial_path = Path::new();
    initial_path.add_cave("start".to_owned());
    let mut to_search = vec![initial_path];

    while let Some(path) = to_search.pop() {
        let curr_cave = path.caves.last().unwrap();
        if curr_cave == "end" {
            paths.push(path.clone());
            continue;
        }

        if let Some(curr_cave_conns) = conns.conn.get(curr_cave) {
            for connected_cave in curr_cave_conns {
                if (&connected_cave.to_uppercase() == connected_cave) || !path.contains(connected_cave) {
                    let mut new_path = path.clone();
                    new_path.add_cave(connected_cave.to_owned());
                    to_search.push(new_path);
                }
            }
        }
    }

    paths
}

fn find_paths_part_2(conns: &Connections) -> Vec<Path> {
    let mut paths = vec![];
    let mut initial_path = Path::new();
    initial_path.add_cave("start".to_owned());
    let mut to_search = vec![initial_path];

    while let Some(path) = to_search.pop() {
        let curr_cave = path.caves.last().unwrap();
        if curr_cave == "end" {
            paths.push(path.clone());
            continue;
        }

        if let Some(curr_cave_conns) = conns.conn.get(curr_cave) {
            for connected_cave in curr_cave_conns {
                if connected_cave == "start" {
                    continue;
                }
                if (&connected_cave.to_uppercase() == connected_cave) || !path.contains(connected_cave) {
                    let mut new_path = path.clone();
                    new_path.add_cave(connected_cave.to_owned());
                    to_search.push(new_path);
                } else if !path.double_visited {
                    let mut new_path = path.clone();
                    new_path.add_cave(connected_cave.to_owned());
                    new_path.double_visited = true;
                    to_search.push(new_path);
                }
            }
        }
    }

    paths
}


fn load_input_lines() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut lines = Vec::new();
    for l in reader.lines().map(|l| l.unwrap()) {
        lines.push(l);
    }

    Ok(lines)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_lines = load_input_lines()?;
    let conns = parse_input_lines_into_caves(&input_lines);

    let paths_part1 = find_paths_part_1(&conns);
    // println!("part1: paths: {:?}", paths_part1);
    println!("part1: number of paths: {}", paths_part1.len());

    let paths_part2 = find_paths_part_2(&conns);
    // println!("part1: paths: {:?}", paths_part2);
    println!("part2: number of paths: {}", paths_part2.len());
    Ok(())
}