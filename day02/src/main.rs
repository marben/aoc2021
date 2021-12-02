use std::io::BufRead;

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

struct Position {
    forward: i64,
    depth: i64,
}

fn position_after_commands_part1(commands: &Vec<Command>) -> Position {
    let mut forward = 0;
    let mut depth = 0;
    for cmd in commands {
        match cmd {
            Command::Forward(x) => forward += x,
            Command::Down(d) => depth += d,
            Command::Up(u) => depth -= u,
        }
    }

    Position { forward, depth }
}

fn position_after_commands_part2(commands: &Vec<Command>) -> Position {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in commands {
        match cmd {
            Command::Up(u) => aim -= u,
            Command::Down(d) => aim += d,
            Command::Forward(x) => {
                forward += x;
                depth += aim * x;
            }
        }
    }

    Position { forward, depth }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut commands = Vec::new();
    for l in reader.lines() {
        let line = l?;
        let mut split_iter = line.split(" ");
        let command_str = split_iter.next().ok_or("couldn't parse command")?;
        let amount = split_iter.next().ok_or("couldn't parse amount")?.parse::<i64>()?;
        let command = match command_str {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => panic!("unexpected command {}", command_str),
        };
        commands.push(command);
    }

    let pos1 = position_after_commands_part1(&commands);
    println!("position 1 after commands: x={} d={}", pos1.forward, pos1.depth);
    println!("position 1 and depth multiplied: {}", pos1.forward * pos1.depth);

    let pos2 = position_after_commands_part2(&commands);
    println!("position 2 after commands: x={} d={}", pos2.forward, pos2.depth);
    println!("position 2 and depth multiplied: {}", pos2.forward * pos2.depth);

    Ok(())
}
