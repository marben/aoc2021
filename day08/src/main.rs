use std::collections::{HashMap, HashSet};
use std::io::BufRead;

struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

fn load_input() -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut lines_parsed = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let input_output_split: Vec<String> = line.split("|").map(|s| s.to_string()).collect();
        assert_eq!(input_output_split.len(), 2);

        let input_line = &input_output_split[0];
        let input_parsed = input_line.trim().split_whitespace().map(|s| s.to_string()).collect();

        let output_line = &input_output_split[1];
        let output_parsed = output_line.trim().split_whitespace().map(|s| s.to_string()).collect();

        lines_parsed.push(Line { input: input_parsed, output: output_parsed });
    }

    Ok(lines_parsed)
}

fn part_1(lines: &Vec<Line>) -> u32 {
    let mut sum = 0;
    for l in lines {
        for s in &l.output {
            sum += match s.len() {
                2 | 4 | 3 | 7 => 1, // no of segments of digits 1, 4, 7, 8
                _ => 0
            }
        }
    }
    sum
}

// each line always contains all numbers 0..10, so we don't need to write a generic algorithm
// instead we just follow simple process, to figure out the signals from easiest to more complicated
fn decode_line(line: &Line) -> Result<i32, Box<dyn std::error::Error>> {
    let in_str_v: &Vec<String> = &line.input;
    let mut digits_inputs_sets = HashMap::new();
    for in_word in in_str_v {
        let s: HashSet<_> = HashSet::from_iter(in_word.chars());
        match digits_inputs_sets.get_mut(&s.len()) {
            None => { digits_inputs_sets.insert(s.len(), vec![s]); }
            Some(v) => { v.push(s); }
        }
    }

    // we take number one, which has 2 signals
    let s1 = digits_inputs_sets.get(&2).unwrap().get(0).unwrap();
    // println!("Letter 1 set: {:?}", s1);

    let s7 = digits_inputs_sets.get(&3).unwrap().get(0).unwrap();
    // println!("Letter 7 set: {:?}", s7);

    // difference of 1 and 7 is signal mapping for a
    // let ma: HashSet<_> = s7.difference(s1).cloned().collect();
    // println!("mapping for a is: {:?}", ma);

    // vector of sets of digits 2, 3 and 5
    let vs235 = digits_inputs_sets.get(&5).unwrap();
    // the digit that fully contains signals from s1 is 3
    let mut s3 = &HashSet::new();
    let mut vs25: Vec<&HashSet<char>> = Vec::new();
    for s in vs235 {
        let intersection: HashSet<_> = s.intersection(s1).cloned().collect();
        // println!("intersection is: {:?}", intersection);
        if intersection.len() == 2 {
            s3 = s;
        } else {
            vs25.push(s);
        }
    }
    // println!("s3 = {:?}", s3);
    let s4 = digits_inputs_sets.get(&4).unwrap().get(0).unwrap();
    // println!("s4 = {:?}", s4);
    // {s4} - {s3} gives us mapping for signal b
    let mb: HashSet<_> = s4.difference(&s3).copied().collect();
    // println!("mb = {:?}", mb);

    // out of vs25 the one that contains {b} is 5
    let mut s5: HashSet<char> = HashSet::new();
    let mut s2: HashSet<_> = HashSet::new();
    for s in &vs25 {
        if s.intersection(&mb).collect::<HashSet<&char>>().len() == 1 {
            s5 = s.iter().copied().collect();
        } else {
            s2 = s.iter().copied().collect();
        }
    }

    // println!("s5 is {:?}", s5);

    // signal c is {1} - {5}
    let mc: HashSet<_> = s1.difference(&s5).cloned().collect();
    // println!("mc = {:?}", mc);

    // md is s4 - s1 - mb
    let md: HashSet<_> = s4.difference(&s1).cloned().collect::<HashSet<_>>().difference(&mb).cloned().collect();

    // println!("md is {:?}", md);

    // me is s2 - s3
    let me: HashSet<_> = s2.difference(&s3).cloned().collect();
    // println!("me is {:?}", me);

    // mf is s1 - s2
    // let mf: HashSet<_> = s1.difference(&s2).cloned().collect();
    // println!("mf is {:?}", mf);

    // mg is s2 - s7 - md - me
    // let mg: HashSet<_> = s2.difference(&s7).cloned().collect::<HashSet<_>>().difference(&md).cloned().collect::<HashSet<_>>().difference(&me).cloned().collect();
    // println!("mg is {:?}", mg);

    // so far we have s1, s2, s3, s4, s5, s7
    // let's get the rest
    let s8: HashSet<_> = digits_inputs_sets.get(&7).unwrap().get(0).unwrap().clone();
    // println!("s8 is: {:?}", s8);

    let s6: HashSet<_> = s8.difference(&mc).cloned().collect();
    // println!("s6 is {:?}", s6);

    let s0: HashSet<_> = s8.difference(&md).cloned().collect();
    // println!("s0 is {:?}", s0);

    let s9: HashSet<_> = s8.difference(&me).cloned().collect();
    // println!("s9 is {:?}", s9);

    // since we have the hashsets with all numbers, lets just compare them with the output

    let mut out_num = 0;
    for o in &line.output {
        let o_set: HashSet<_> = HashSet::from_iter(o.chars());
        let n = match o_set {
            x if x == s0 => 0,
            x if x == *s1 => 1,
            x if x == s2 => 2,
            x if x == *s3 => 3,
            x if x == *s4 => 4,
            x if x == s5 => 5,
            x if x == s6 => 6,
            x if x == *s7 => 7,
            x if x == s8 => 8,
            x if x == s9 => 9,
            _ => {panic!("failed to mach output")}
        };

        out_num *= 10;
        out_num += n;
        // println!("the number is {}", n)
    }

    Ok(out_num)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("number signals:\n{:?}", number_signals());

    let lines = load_input()?;

    let s1 = part_1(&lines);
    println!("part 1 solution: {}", s1);

    let mut sum = 0;
    for l in &lines {
        let n = decode_line(l)?;
        sum += n;
    }
    println!("part 2 solution: {}", sum);

    Ok(())
}
