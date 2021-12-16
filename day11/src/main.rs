use std::fmt::{Display, Formatter};
use std::io::BufRead;

struct Map {
    v: Vec<Vec<Octopus>>,
    x_dim: i32,
    y_dim: i32,
    total_flashes_number: u64,
    current_step_flashes_number: u64,
    step: u32,
}

struct Octopus {
    energy: i32,
    flashed: bool,
}

impl Map {
    fn new(lines: &Vec<String>) -> Map {
        let mut v = Vec::new();
        let mut x_dim = 0;

        for l in lines.iter() {
            let vi: Vec<i32> = l.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect();
            let mut vo: Vec<Octopus> = Vec::new();
            for e in vi.iter().cloned() {
                vo.push(Octopus { energy: e, flashed: false })
            }

            x_dim = x_dim.max(vo.len() as i32);
            v.push(vo);
        }

        let y_dim = v.len();

        Map { v, x_dim, y_dim: y_dim as i32, total_flashes_number:0, current_step_flashes_number:0, step:0 }
    }

    fn increment_all(&mut self) {
        for x in 0..self.x_dim {
            for y in 0..self.y_dim {
                let o = self.get_octopus_mut(x, y).unwrap();
                o.energy += 1;
            }
        }
    }

    fn get_octopus_mut(&mut self, x: i32, y: i32) -> Option<&mut Octopus> {
        if x < 0 || x >= self.x_dim || y < 0 || y >= self.y_dim {
            return None;
        }

        Some(&mut self.v[y as usize][x as usize])
    }

    // flashes the octupus if it's energy > 9 and increments and flashes adjacent octopuses
    // the field is quite small, so we use recursion to flash the neighbours
    fn attempt_flash(&mut self, x: i32, y: i32) {
        if let Some(o) = self.get_octopus_mut(x, y) {
            if o.energy > 9 && !o.flashed {
                // FLASH
                o.flashed = true;
                self.total_flashes_number += 1;
                self.current_step_flashes_number += 1;
                for x in [x-1, x, x+1] {
                    for y in [y-1, y, y+1] {
                        if let Some(o) = self.get_octopus_mut(x,y) {
                            o.energy += 1;
                            self.attempt_flash(x, y);
                        }
                    }
                }
            }
        }
    }

    fn flash_all(&mut self) {
        for x in 0..self.x_dim {
            for y in 0..self.y_dim {
                self.attempt_flash(x, y);
            }
        }
    }

    fn zero_out_flashed_octopuses(&mut self) {
        for x in 0..self.x_dim {
            for y in 0..self.y_dim {
                if let Some(o) = self.get_octopus_mut(x, y) {
                    if o.flashed {
                        o.energy = 0;
                        o.flashed = false;
                    }
                }
            }
        }
        self.current_step_flashes_number = 0;
    }

    // returns true if all octopuses flashed
    fn step(&mut self) -> bool {
        // first we increment all octopuses
        self.increment_all();
        self.flash_all();
        self.step += 1;
        if self.current_step_flashes_number == self.x_dim as u64 * self.y_dim as u64 {
            return true;
        }
        self.zero_out_flashed_octopuses();
        return false;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in &self.v {
            for n in l {
                write!(f, "{}", n.energy)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input()?;
    let mut map = Map::new(&lines);
    for _ in 0..100 {
        if map.step() {
            println!("all octopuses flashed during step: {}", map.step);
        }
    }
    println!("flashes after 100 steps: {}", map.total_flashes_number);

    let test_steps = 10000;
    for _ in 0..10000-100 {
        if map.step() {
            println!("all octopuses flashed together during step: {}", map.step);
            return Ok(());
        }
    }

    println!("couldn't find, when all octopuses flash together within first {} steps", test_steps);

    Ok(())
}

fn load_input() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut lines = Vec::new();
    for l in reader.lines().map(|l| l.unwrap()) {
        lines.push(l);
    }

    Ok(lines)
}