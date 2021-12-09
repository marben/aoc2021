use std::fmt::{Display, Formatter};
use std::io::BufRead;

const MAP_SIDE: usize = 1000;

struct Coord {
    x: i32,
    y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Line(Coord, Coord);

impl Line {
    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}

struct Map {
    field: Vec<Vec<i32>>,
}

impl Map {
    fn new(dim: usize) -> Map {
        Map {
            field: vec![vec![0; dim]; dim]
        }
    }

    fn mark_point(&mut self, x: i32, y: i32) {
        self.field[x as usize][y as usize] += 1;
    }

    // adds +1 to every 'pixel' that the line hits
    fn mark_line(&mut self, line: &Line) {
        // find the longer distance
        let dx = (line.1.x - line.0.x).abs();
        let dy = (line.1.y - line.0.y).abs();

        if dx >= dy {
            // iterating along x
            let x_start;
            let y_start;
            let y_inc: f32;
            if line.1.x > line.0.x {
                x_start = line.0.x;
                y_start = line.0.y;
                y_inc = (line.1.y - line.0.y) as f32 / dx as f32;
            } else {
                x_start = line.1.x;
                y_start = line.1.y;
                y_inc = (line.0.y - line.1.y) as f32 / dx as f32;
            }

            let mut y = y_start as f32;
            for x in x_start..x_start + dx + 1 {
                self.mark_point(x, y.round() as i32);
                y += y_inc;
            }
        } else {
            // iterating along y
            let x_start;
            let y_start;
            let x_inc: f32;
            if line.1.y > line.0.y {
                x_start = line.0.x;
                y_start = line.0.y;
                x_inc = (line.1.x - line.0.x) as f32 / dy as f32;
            } else {
                x_start = line.1.x;
                y_start = line.1.y;
                x_inc = (line.0.x - line.1.x) as f32 / dy as f32;
            }

            let mut x = x_start as f32;
            for y in y_start..y_start + dy + 1 {
                self.mark_point(x.round() as i32, y);
                x += x_inc;
            }
        }
    }

    fn coords_larger_than_2(&self) -> Vec::<Coord> {
        let mut v = Vec::new();
        for x in 0..self.field.len() {
            for y in 0..self.field.len() {
                if self.field[x][y] >= 2 {
                    v.push(Coord { x: x as i32, y: y as i32 });
                }
            }
        }
        v
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.field.len() {
            for x in 0..self.field.len() {
                if self.field[x][y] == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", self.field[x][y])?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn load_input() -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut lines = Vec::new();
    for l in reader.lines().map(|l| l.unwrap()) {
        let line = parse_line(&l)?;
        lines.push(line);
    }

    Ok(lines)
}

fn parse_line(line_str: &str) -> Result<Line, Box<dyn std::error::Error>> {
    let split_line: Vec<&str> = line_str.split("->").collect();
    if split_line.len() != 2 {
        return Err(format!("unexpected split_line len: {}", split_line.len()).into());
    }
    let c1 = parse_coordinates(split_line[0])?;
    let c2 = parse_coordinates(split_line[1])?;

    Ok(Line(c1, c2))
}

fn parse_coordinates(coords_str: &str) -> Result<Coord, Box<dyn std::error::Error>> {
    let coords_split: Vec<&str> = coords_str.split(",").map(|s| s.trim()).collect();
    if coords_split.len() != 2 {
        return Err(format!("unexpected coords_split len: {}", coords_split.len()).into());
    }
    let x = coords_split[0].parse::<i32>()?;
    let y = coords_split[1].parse::<i32>()?;

    Ok(Coord { x, y })
}

fn part_1(lines: &Vec<Line>) -> usize {
    let mut map = Map::new(MAP_SIDE);
    let horizontal_and_vertical_lines: Vec<&Line> = lines.iter().filter(|l| l.is_horizontal() || l.is_vertical()).collect();

    for l in horizontal_and_vertical_lines.iter() {
        map.mark_line(&l);
    }

    return map.coords_larger_than_2().len();
}

fn part_2(lines: &Vec<Line>) -> usize {
    let mut map = Map::new(MAP_SIDE);
    for l in lines.iter() {
        map.mark_line(&l);
    }

    return map.coords_larger_than_2().len();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input()?;

    println!("result for part 1: {}", part_1(&lines));
    println!("result for part 2: {}", part_2(&lines));

    Ok(())
}
