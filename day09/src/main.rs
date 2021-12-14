use std::cmp;
use std::collections::HashSet;
use std::io::BufRead;

struct Map {
    v: Vec<Vec<i32>>,
    x: i32,
    y: i32,
}

impl Map {
    fn new() -> Map {
        Map { v: Vec::new(), x: 0, y: 0 }
    }

    fn add_row(&mut self, r: Vec<i32>) {
        self.x = cmp::max(self.x, r.len() as i32);
        self.y += 1;
        self.v.push(r);
    }

    // returns coord of local minimum or None, if it is not local minimum
    fn is_local_minimum(&self, x: i32, y: i32) -> Option<Coord> {
        let c = match self.get_coord(x, y) {
            Some(c) => c,
            None => { return None; }
        };

        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some(c2) = self.get_coord(x, y) {
                if c2.d <= c.d {
                    return None;
                }
            }
        }

        Some(c)
    }

    // can ask outside of map, will get None
    fn get_coord(&self, x: i32, y: i32) -> Option<Coord> {
        if x < 0 || y < 0 || x >= self.x || y >= self.y {
            return None;
        }
        let d = self.v[y as usize][x as usize];
        Some(Coord { x, y, d })
    }
}

fn load_input() -> Result<Map, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut m = Map::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let v = line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        m.add_row(v);
    }

    Ok(m)
}

// returns the "risk level"
fn part_1(local_minimums: &Vec<Coord>) -> i32 {
    // risk level is depth + 1
    local_minimums.iter().map(|c| c.d + 1).sum()
}

fn find_local_minimums(m: &Map) -> Vec<Coord> {
    let mut mins = Vec::new();
    for x in 0..m.x {
        for y in 0..m.y {
            if let Some(c) = m.is_local_minimum(x, y) {
                mins.push(c)
            }
        }
    }
    mins
}

fn find_basin(map: &Map, local_minimum: &Coord) -> Vec<Coord> {
    let mut processed = HashSet::<Coord>::new();
    let mut to_process = vec![*local_minimum];
    let mut basin: HashSet<Coord> = HashSet::new();

    while let Some(c) = to_process.pop() {
        processed.insert(c);
        basin.insert(c);

        for (x, y) in [(c.x - 1, c.y), (c.x + 1, c.y), (c.x, c.y - 1), (c.x, c.y + 1)] {
            if let Some(c2) = map.get_coord(x, y) {
                if !processed.contains(&c2) {
                    if (c2.d > c.d) && (c2.d != 9) {
                        to_process.push(c2);
                    }
                }
            }
        }
    }

    basin.into_iter().collect()
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
    d: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = load_input()?;

    let local_minimums = find_local_minimums(&map);
    println!("part 1 result: {}", part_1(&local_minimums));

    let mut basin_sizes = Vec::new();
    for min in &local_minimums {
        let b = find_basin(&map, min);
        basin_sizes.push(b.len());
        // println!("found basin with size {}",b.len());
    }
    basin_sizes.sort();
    let mult = basin_sizes[basin_sizes.len() - 1] * basin_sizes[basin_sizes.len() - 2] * basin_sizes[basin_sizes.len() - 3];
    println!("part 2 result: {}", mult);

    Ok(())
}
