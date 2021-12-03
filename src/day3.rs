#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|l| l.to_string())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let mut sb = String::new();
    let mut en = String::new();
    let lines = input.len();
    let width = input.get(0).unwrap().len();

    for i in 0..width {
        let mut ones = 0;
        for j in 0..lines {
            if input.get(j).unwrap().chars().nth(i).unwrap() == '1' {
                ones+=1;
            }
        }

        println!("ones {}", ones);
        if ones > (lines/2) {
            sb.push_str("1");
            en.push_str("0");
        } else {
            sb.push_str("0");
            en.push_str("1");
        }
    }
    
    println!("{} * {}", sb, en);
    // This wasn't printing anything so I just printed the binary numbers and 
    // used an online binary calculator lmao
    isize::from_str_radix(sb.as_str(), 2).unwrap() as i32 * isize::from_str_radix(en.as_str(), 2).unwrap() as i32
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let mut inp = Vec::new();
    inp.clone_from(&input.to_vec());
    let mut holder1 = Vec::new();
    let mut holder2 = Vec::new();

    let mut i = 0;
    while inp.len() > 1 {
        holder1.clear(); holder2.clear();
        let mut ones = 0;
        let mut zeros = 0;
        for l in &inp {
            let b = l.chars().nth(i).unwrap();
            if b == '1' {
                ones += 1;
                holder1.push(l.clone());
            } else {
                zeros += 1;
                holder2.push(l.clone());
            }
        }

        if ones >= zeros {
            inp.clear();
            inp = holder1.clone();
        } else {
            inp.clear();
            inp = holder2.clone();
        }
        //println!("Step: {:?}", &inp);
        
        i += 1;
    }

    println!("{}", inp.get(0).unwrap());

    inp.clone_from(&input.to_vec());

    let mut i = 0;
    while inp.len() > 1 {
        holder1.clear(); holder2.clear();
        let mut ones = 0;
        let mut zeros = 0;
        for l in &inp {
            let b = l.chars().nth(i).unwrap();
            if b == '1' {
                ones += 1;
                holder1.push(l.clone());
            } else {
                zeros += 1;
                holder2.push(l.clone());
            }
        }

        if ones < zeros {
            inp.clear();
            inp = holder1.clone();
        } else {
            inp.clear();
            inp = holder2.clone();
        }
        //println!("Step: {:?}", &inp);
        
        i += 1;
    }

    println!("{}", inp.get(0).unwrap());

    0
}
