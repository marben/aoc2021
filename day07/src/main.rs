use std::io::BufRead;

fn load_input() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let mut reader = std::io::BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let v = line.trim().split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    Ok(v)
}

fn cost_of_alignment_part_1(crabs: &Vec<u32>, pos: u32) -> i64 {
    crabs.iter().map(|c| (pos as i64 - *c as i64).abs()).sum()
}

fn cost_of_alignment_part_2(crabs: &Vec<u32>, pos: u32) -> i64 {
    let mut sum = 0;
    for c in crabs {
        let d = (pos as i64 - *c as i64).abs();
        let cost = (d * (d + 1)) / 2;
        sum += cost;
    }
    sum
}

fn costs(crab_positions: &Vec<u32>, cost_func: fn(&Vec<u32>, u32) -> i64) -> Vec<i64> {
    let max_position = crab_positions.iter().max().unwrap();

    let mut costs = Vec::new();
    for pos in 0..*max_position {
        costs.push(cost_func(crab_positions, pos as u32));
    }

    costs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crab_positions = load_input()?;

    // part 1
    let part1_costs = costs(&crab_positions, cost_of_alignment_part_1);
    let part1_min_cost = part1_costs.iter().min().unwrap();
    println!("part 1 minimal cost: {}", part1_min_cost);

    // part 2
    let part2_costs = costs(&crab_positions, cost_of_alignment_part_2);
    let part2_min_cost = part2_costs.iter().min().unwrap();
    println!("part 2 minimal cost: {}", part2_min_cost);

    Ok(())
}
