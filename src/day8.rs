use itertools::Itertools;

pub struct Input {
    signal: Vec<String>,
    output: Vec<String>
}

#[aoc_generator(day8)]
pub fn generate_day8(input: &str) -> Vec<Input> {
    input.lines()
        .map(|line| {
            let signal = line.split_whitespace().take(10).map(|s| s.to_string()).collect();
            let output = line.split_whitespace().skip(11).map(|s| s.to_string()).collect();

            Input { signal, output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &[Input]) -> usize {
    let mut answer = 0;

    for line in data {
        answer += line.output.iter()
            .map(|s| s.len())
            .filter(|&s| s == 2 || s == 3 || s == 4 || s == 7)
            .count();
    }

    answer
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &[Input]) -> i32 {
    let mut answer = 0;

    for line in data {
        // Closures
        let get_num_iter = |n_size| line.signal.iter().filter(move |n| n.len() == n_size);
        let count_matches = |src: &String, dest: &String| src.chars().filter(|c| dest.chars().find(|ch| c == ch ).is_some()).count(); 

        // Easy numbers
        let eight = get_num_iter(7).next().unwrap();
        let one = get_num_iter(2).next().unwrap();
        let seven = get_num_iter(3).next().unwrap();
        let four = get_num_iter(4).next().unwrap();

        // Get three to differentiate between 6 & 9
        let three = get_num_iter(5).filter(|s| count_matches(s, seven) == 3 ).next().unwrap();

        let nine = get_num_iter(6).filter(|s| count_matches(s, three) == 5 ).next().unwrap();
        let six = get_num_iter(6).filter(|s| count_matches(s, one) == 1 ).next().unwrap();
        let five = get_num_iter(5).filter(|s| count_matches(s, six) == 5 ).next().unwrap();
        let two = get_num_iter(5).filter(|s| count_matches(s, nine) == 4 ).next().unwrap();

        // Can't figure out a clean way to find zero, so it's just the only one remaining
        let zero = get_num_iter(6).filter(|&s| s != nine && s != six).next().unwrap();

        let numbers = vec![zero, one, two, three, four, five, six, seven, eight, nine];
        println!("Numbers: {:?}", &numbers);

        let output = line.output.iter()
            .map(|num| numbers.iter().position(|n| n.chars().sorted().eq(num.chars().sorted())).unwrap())
            .fold(0, |acc, num| (acc * 10) + num) as i32;

        answer += output;
        println!("Output: {}", output);
    }

    answer
}
