use std::collections::HashSet;

type Array2D<T> = Vec<Vec<T>>;

#[aoc_generator(day9)]
pub fn generate_day9(input: &str) -> Array2D<u32> {
    input.lines().map(|l| {
        l.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<u32>>()
    }).collect()
}

fn get_from_array<T>(arr: &Array2D<T>, x: usize, y: usize) -> Option<&T> {
    if let Some(row) = arr.get(x) {
        row.get(y)
    } else {
        None
    }
}

#[aoc(day9, part1)]
pub fn day9_part1(data: &Array2D<u32>) -> u32 {
    let mut answer = 0;

    for x in 0..data.len() {
        for y in 0..data[x].len() {
            let point = get_from_array(data, x, y).unwrap();

            let top = get_from_array(data, x, y-1).unwrap_or(&10);
            let left = get_from_array(data, x-1, y).unwrap_or(&10);
            let right = get_from_array(data, x+1, y).unwrap_or(&10);
            let bottom = get_from_array(data, x, y+1).unwrap_or(&10);

            if point < top && point < left && point < right && point < bottom {
                //println!("Low point: {}", point);
                answer += point + 1;
            }
        }
    }

    answer
}

#[aoc(day9, part2)]
pub fn day9_part2(data: &Array2D<u32>) -> u32 {
    let mut set_sizes: Vec<usize> = Vec::new();
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            let point = get_from_array(data, x, y).unwrap();

            let top = get_from_array(data, x, y-1).unwrap_or(&10);
            let left = get_from_array(data, x-1, y).unwrap_or(&10);
            let right = get_from_array(data, x+1, y).unwrap_or(&10);
            let bottom = get_from_array(data, x, y+1).unwrap_or(&10);

            if point < top && point < left && point < right && point < bottom {
                let mut flood: HashSet<(usize, usize)> = HashSet::new();
                let mut stack: Vec<(usize, usize)> = Vec::new();

                stack.push((x, y));

                while !stack.is_empty() {
                    let pos = stack.pop().unwrap();
                    if let Some(_) = get_from_array(data, pos.0, pos.1) {
                        if flood.contains(&pos) {
                            continue;
                        }

                        flood.insert(pos);
                        //println!("Added: ({}, {})", pos.0, pos.1);

                        let mut q = |x_offset: i32, y_offset: i32| {
                            let x = pos.0 as i32 + x_offset;
                            let y = pos.1 as i32 + y_offset;
                            let offset = get_from_array(data, x as usize, y as usize);
                            if offset.is_some() && offset.unwrap() < &9 {
                                stack.push((x as usize, y as usize));
                            }
                        };

                        q(0, -1);
                        q(-1, 0);
                        q(1, 0);
                        q(0, 1);

                    }
                }
                set_sizes.push(flood.len());
                //println!("Flood size: {}", flood.len());
            }
        }
    }

    set_sizes.sort();
    set_sizes.iter().rev().take(3)
        .fold(1, |acc, &n| acc * (n as u32))
}
