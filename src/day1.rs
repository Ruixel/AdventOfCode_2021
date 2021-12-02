#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    /*
    input.iter().skip(1).zip(input.iter())
        .fold(0, |acc, (n, prev)| if n > prev { acc + 1 } else { acc })
    */

    input.windows(2).filter(|&n| n[1] > n[0]).count() as i32
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    /* Vomit
    let prev_zip: Vec<i32> = input.iter().skip(3).zip(input.iter().skip(2).zip(input.iter().skip(1))).map(|(n, (n1, n2))| n + n1 + n2).collect();
    let zip: Vec<i32> = input.iter().skip(2).zip(input.iter().skip(1).zip(input.iter())).map(|(n, (n1, n2))| n + n1 + n2).collect();

    prev_zip.iter().zip(zip.iter())
        .fold(0, |acc, (n, prev)| if n > prev { acc + 1 } else { acc })
    */

    input.windows(4).filter(|&n| n[3] > n[0]).count() as i32
}
