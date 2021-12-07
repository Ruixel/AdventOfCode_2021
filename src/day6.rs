#[aoc_generator(day6)]
pub fn day6_generator(input: &str) -> Vec<i32> {
    input.split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn day6_part1(data: &[i32]) -> i32 {
    let mut fishes = data.to_vec();

    for _day in 0..80 {
        //println!("Day {}: {:?}", day, &fishes);

        let mut addition = 0;
        fishes = fishes.iter()
            .map(|&f| { if f == 0 { addition += 1; 6 } else { f - 1 } })
            .collect();

        for _i in 0..addition {
            fishes.push(8);
        }
        
        //println!("Day {}: {:?}", day, fishes.iter().filter(|&&n| n==0).count());
    }

    fishes.len() as i32
}

#[aoc(day6, part2)]
pub fn day6_part2(data: &[i32]) -> u128 {
    let mut fishes: [u128; 9] = [0; 9];

    data.iter()
        .for_each(|&n| fishes[n as usize] += 1);

    for _day in 0..256 {
        let new_fish = fishes[0];
        for i in 1..=8 {
            fishes[i-1] = fishes[i];
        }

        fishes[8] = new_fish;
        fishes[6] += new_fish;
    }

    fishes.iter().fold(0, |acc, n| acc + n) 
}
