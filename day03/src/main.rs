use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bitsets_lines = load_input()?;

    // part 1
    let (e, g) = calculate_gamma_and_epsilon(&bitsets_lines);
    println!("e * g = {}", e * g);

    // part 2
    let o2_gen_rating = calculate_oxygen_generator_rating(&bitsets_lines)?;
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&bitsets_lines)?;
    println!("life support rating: {}", o2_gen_rating * co2_scrubber_rating);

    Ok(())
}

fn load_input() -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);
    let mut lines_bits = Vec::new();

    for l in reader.lines() {
        let mut line_bits: Vec<u32> = Vec::new();
        for c in l?.chars() {
            let bit = match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("unexpected character '{}'", c),
            };
            line_bits.push(bit);
        }
        lines_bits.push(line_bits);
    }

    Ok(lines_bits)
}

fn calculate_oxygen_generator_rating(bitset_lines: &Vec<Vec<u32>>) -> Result<u32, String>{
    let mut bitsets_left = bitset_lines.to_vec();
    for pos in 0..bitset_lines.len() {
        let (ones, zeros) = calculate_ones_and_zeroes_sums_on_position_in_list_of_bitsets(&bitsets_left, pos);
        if zeros > ones {
            bitsets_left = bitsets_left.into_iter().filter(|bs| bs[pos]==0).collect();
        } else {
            bitsets_left = bitsets_left.into_iter().filter(|bs| bs[pos]==1).collect();
        }
        if bitsets_left.len() == 1 {
            return Ok(bitset_to_number(&bitsets_left[0]));
        }
    }

    Err(format!("expected to find oxygen generator rating, but there are still {} bitsets left", bitsets_left.len()))
}

fn calculate_co2_scrubber_rating(bitset_lines: &Vec<Vec<u32>>) -> Result<u32, String> {
    let mut bitsets_left = bitset_lines.to_vec();
    for pos in 0..bitset_lines.len() {
        let (ones, zeros) = calculate_ones_and_zeroes_sums_on_position_in_list_of_bitsets(&bitsets_left, pos);
        if zeros <= ones {
            bitsets_left = bitsets_left.into_iter().filter(|bs| bs[pos]==0).collect();
        } else {
            bitsets_left = bitsets_left.into_iter().filter(|bs| bs[pos]==1).collect();
        }
        if bitsets_left.len() == 1 {
            return Ok(bitset_to_number(&bitsets_left[0]));
        }
    }

    Err(format!("expected to find co2 scrubber rating, but there are still {} bitsets left", bitsets_left.len()))
}

fn calculate_gamma_and_epsilon(bitset_lines: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();

    for (pos, _) in bitset_lines[0].iter().enumerate() {
        let (ones, zeros) = calculate_ones_and_zeroes_sums_on_position_in_list_of_bitsets(bitset_lines, pos);
        if ones > zeros {
            gamma_bits.push(1);
            epsilon_bits.push(0);
        } else {
            gamma_bits.push(0);
            epsilon_bits.push(1)
        }
    }

    let g = bitset_to_number(&gamma_bits);
    let e = bitset_to_number(&epsilon_bits);

    (g,e)
}

fn calculate_ones_and_zeroes_sums_on_position_in_list_of_bitsets(bs_list: &Vec<Vec<u32>>, pos: usize) -> (u32, u32) {
    let mut ones = 0;
    let mut zeroes = 0;
    for bs in bs_list {
        if bs[pos] == 1 {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }

    (ones, zeroes)
}

fn bitset_to_number(in_vec: &Vec<u32>) -> u32 {
    let mut o = 0;
    for (i, b) in in_vec.iter().enumerate() {
        o |= b << in_vec.len() - 1 - i;
    }
    o
}