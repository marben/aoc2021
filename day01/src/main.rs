use std::{fs, io};
use std::io::BufRead;

fn count_dips(depths: &[i64]) -> i64 {
    let mut dips = 0;
    let mut d_prev = depths[0]; // this can panic on empty vec
    for d in depths[1..].iter() {
        if d_prev < *d {
            dips += 1;
        }
        d_prev = *d;
    }
    dips
}

fn calculate_3_windows_depths(depths: &[i64]) -> Vec<i64> {
    let mut w3_depths = Vec::new();
    for (i, _) in depths[..depths.len() - 2].iter().enumerate() {
        w3_depths.push(depths[i] + depths[i + 1] + depths[i + 2]);
    }

    w3_depths
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = fs::File::open("input")?;

    let mut depths = Vec::new();
    let reader = io::BufReader::new(f);
    for l in reader.lines() {
        let line = l?;
        let line_int: i64 = line.parse::<i64>()?;
        depths.push(line_int);
        // println!("{}", line_int);
    }

    // Part 1
    let dips_num = count_dips(&depths);
    println!("part1 - dips number: {}", dips_num);

    // Part2
    let w3_depths = calculate_3_windows_depths(&depths);
    let w3_dips = count_dips(&w3_depths);
    println!("part2 - w3 dips number:{}", w3_dips);

    Ok(())
}
