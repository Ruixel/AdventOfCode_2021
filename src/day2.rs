pub struct Command {
    command: String,
    num: i32
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input.lines()
        .map(|l| {
            let mut iter = l.trim().split_whitespace();
            Command {
                command: iter.next().unwrap().to_string(),
                num: iter.next().unwrap().parse().unwrap()
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> i32 {
    let mut x = 0; let mut depth = 0;

    for c in input {
        match c.command.as_str() {
            "forward" => x += c.num,
            "down" => depth += c.num,
            "up" => depth -= c.num,
            _ => println!("Unknown")
        }
    }
    
    x * depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> i32 {
    let mut x = 0; let mut depth = 0; let mut aim = 0;

    for c in input {
        match c.command.as_str() {
            "forward" => {
                x += c.num;
                depth += aim * c.num
            },
            "down" => aim += c.num,
            "up" => aim -= c.num,
            _ => println!("Unknown")
        }
    }
    
    x * depth
}
