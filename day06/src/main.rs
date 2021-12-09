use std::io::BufRead;

struct FishPopulation {
    fishes_with_timers: Vec::<u64>,
}

impl FishPopulation {
    fn new() -> FishPopulation {
        return FishPopulation { fishes_with_timers: vec![0; 9] };
    }
    fn add_fish(&mut self, timer: u64) {
        self.fishes_with_timers[timer as usize] += 1;
    }
    fn age(&mut self) {
        let mut v = vec![0; 9];
        for t in 0..8 {
            v[t] = self.fishes_with_timers[t + 1];
        }
        v[8] = self.fishes_with_timers[0];
        v[6] += self.fishes_with_timers[0];
        self.fishes_with_timers = v;
    }
    fn total_number(&self) -> u64 {
        let mut sum = 0;
        for n in &self.fishes_with_timers {
            sum += n;
        }
        sum
    }
}

fn load_input() -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let mut reader = std::io::BufReader::new(f);
    let mut line = "".to_string();
    reader.read_line(&mut line).unwrap();

    let v = line.trim().split(",").map(|s| s.parse::<u64>().unwrap()).collect();

    Ok(v)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial_fishes = load_input()?;

    let mut fishes = FishPopulation::new();
    initial_fishes.iter().for_each(|t| fishes.add_fish(*t));
    println!("fish population: {:?}", fishes.fishes_with_timers);
    for _ in 0..80 {
        fishes.age();
        println!("fish population: {:?}", fishes.fishes_with_timers);
    }

    // part 1
    let it_80 = fishes.total_number();

    // part 2
    for _ in 80..256 {
        fishes.age();
        println!("fish state: {:?}", fishes.fishes_with_timers);
    }

    println!("total population after 80 iterations: {}", it_80);
    println!("total population after 256 iterations: {}", fishes.total_number());

    Ok(())
}
