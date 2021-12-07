#[aoc_generator(day7)]
pub fn day7_generator(input: &str) -> Vec<i32> {
    input.split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn day7_part1(input: &[i32]) -> i32 {
    let crabs = input.to_vec();
    let mut lowest: (i32, i32) = (0, i32::MAX);

    let min = *crabs.iter().min().unwrap() as usize;
    let max = *crabs.iter().max().unwrap() as usize;
    for fuel in min..max {
        let fuel_usage = crabs.iter()
            .map(|&f| (f-fuel as i32).abs())
            .fold(0, |acc, n| acc + n);

        if fuel_usage < lowest.1 {
            lowest = (fuel as i32, fuel_usage);
        }
    }

    lowest.1
}

#[aoc(day7, part2)]
pub fn day7_part2(input: &[i32]) -> i32 {
    let crabs = input.to_vec();
    let mut lowest: (i32, i32) = (0, i32::MAX);

    let min = *crabs.iter().min().unwrap() as usize;
    let max = *crabs.iter().max().unwrap() as usize;
    for fuel in min..max {
        let fuel_usage = crabs.iter()
            .map(|&f| { 
                let n: i32 = (f-fuel as i32).abs();
                n * (n+1)/2 // Triangle numbers o7
            })
            .fold(0, |acc, n| acc + n);

        if fuel_usage < lowest.1 {
            lowest = (fuel as i32, fuel_usage);
        }
    }

    lowest.1
}
