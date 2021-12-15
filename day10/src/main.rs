use std::io::BufRead;

// returns missing closing parentheses as result. wrong char as error
fn parse_line(line: &String) -> Result<Vec<char>, char> {
    // println!("parsing line: {}", line);
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if is_closing_bracket(&c) {
            if let Some(opening) = stack.pop() {
                // println!("found closing {} which should match our opening: {}", c, opening);
                if !opening_matches_closing(&opening, &c) {
                    // println!("{} doesn't match {}", opening, c);
                    return Err(c);
                }
            }
        } else {
            stack.push(c);
        }
    }

    let closing_brackets = gen_closing_brackets(&stack);

    Ok(closing_brackets)
}

fn gen_closing_brackets(openings: &Vec<char>) -> Vec<char> {
    let mut closings = Vec::new();
    for c in openings.iter().rev() {
        closings.push(closing_for_opening(c));
    }

    closings
}

fn calc_closing_score(closings: &Vec<char>) -> i64 {
    let mut sum = 0;
    for c in closings {
        sum *= 5;
        sum += match c {
            &')' => 1,
            &']' => 2,
            &'}' => 3,
            &'>' => 4,
            &_ => 0
        }
    }
    sum
}

fn closing_for_opening(opening: &char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        &_ => '?'
    }
}

fn opening_matches_closing(o: &char, c: &char) -> bool {
    match o {
        '(' => c == &')',
        '[' => c == &']',
        '{' => c == &'}',
        '<' => c == &'>',
        _ => false,
    }
}

fn is_closing_bracket(c: &char) -> bool {
    [')', ']', '}', '>'].contains(c)
}

fn find_average_score(scores: &Vec<i64>) -> i64 {
    let mut s: Vec<i64> = scores.iter().cloned().collect();
    s.sort();

    let avg_index = s.len() / 2;

    s[avg_index]
}

fn calc_err_score(err_chars: &Vec<char>) -> i32 {
    let mut score = 0;
    for c in err_chars {
        score += match *c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        }
    }
    score
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input()?;
    let mut err_chars = Vec::new();
    let mut closing_scores = Vec::new();
    for line in &lines {
        match parse_line(line) {
            Ok(closing) => {
                let closing_score = calc_closing_score(&closing);
                closing_scores.push(closing_score);
            }
            Err(c) => {
                err_chars.push(c);
            }
        }
    }

    println!("part 1 score: {}", calc_err_score(&err_chars));
    println!("part 2 score: {}", find_average_score(&closing_scores));

    Ok(())
}

fn load_input() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut lines = Vec::new();
    for l in reader.lines().map(|l| l.unwrap()) {
        lines.push(l)
    }

    Ok(lines)
}