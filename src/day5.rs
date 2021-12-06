use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Vec2 (i32, i32);

#[derive(Debug)]
pub struct Line {
    start: Vec2,
    end: Vec2
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> Vec<Line> {
    input.lines()
        .map(|l| {
            let mut itr = l.split(" -> ");
            let get_coords = |s: &str| {
                let mut iter = s.split(",");
                let x = iter.next().unwrap().parse::<i32>().unwrap();
                let y = iter.next().unwrap().parse::<i32>().unwrap();
                Vec2(x, y)
            };

            let start = get_coords(itr.next().unwrap());
            let end = get_coords(itr.next().unwrap());

            Line { start, end }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_d5p1(lines: &[Line]) -> i32 {
    let mut set: HashSet<Vec2> = HashSet::new();
    let mut overlaps: HashSet<Vec2> = HashSet::new();

    let mut check_overlap = |pos: Vec2| {
        //println!("Testing: {:?}", &pos);
        match set.get(&pos) {
            Some(_) => overlaps.insert(pos),
            None => set.insert(pos) 
        };
    };

    for line in lines {
        //println!("Line {:?} -> {:?}", &line.start, &line.end);

        if line.start == line.end {
            check_overlap(line.start.clone());
        } else if line.start.0 != line.end.0 && line.start.1 == line.end.1 {
            let delta = line.start.0 - line.end.0;
            let add = -delta / delta.abs();
            for i in 0..=delta.abs() {
                check_overlap(Vec2(line.start.0 + (add*i), line.start.1));
            }
        } else if line.start.1 != line.end.1 && line.start.0 == line.end.0 {
            let delta = line.start.1 - line.end.1;
            let add = -delta / delta.abs();
            for i in 0..=delta.abs() {
                check_overlap(Vec2(line.start.0, line.start.1 + (add*i)));
            }
        }
    }

    overlaps.len() as i32
}

#[aoc(day5, part2)]
pub fn solve_d5p2(lines: &[Line]) -> i32 {
    let mut set: HashSet<Vec2> = HashSet::new();
    let mut overlaps: HashSet<Vec2> = HashSet::new();

    let mut check_overlap = |pos: Vec2| {
        //println!("Testing: {:?}", &pos);
        match set.get(&pos) {
            Some(_) => overlaps.insert(pos),
            None => set.insert(pos) 
        };
    };

    for line in lines {
        //println!("Line {:?} -> {:?}", &line.start, &line.end);

        if line.start == line.end {
            check_overlap(line.start.clone());
        } else if line.start.0 != line.end.0 && line.start.1 == line.end.1 {
            let delta = line.start.0 - line.end.0;
            let add = -delta / delta.abs();
            for i in 0..=delta.abs() {
                check_overlap(Vec2(line.start.0 + (add*i), line.start.1));
            }
        } else if line.start.1 != line.end.1 && line.start.0 == line.end.0 {
            let delta = line.start.1 - line.end.1;
            let add = -delta / delta.abs();
            for i in 0..=delta.abs() {
                check_overlap(Vec2(line.start.0, line.start.1 + (add*i)));
            }
        } else {
            let delta_x = line.start.0 - line.end.0;
            let delta_y = line.start.1 - line.end.1;
            let add_x = -delta_x / delta_x.abs();
            let add_y = -delta_y / delta_y.abs();
            for i in 0..=delta_x.abs() {
                check_overlap(Vec2(line.start.0 + (add_x*i), line.start.1 + (add_y*i)));
            }
        }
    }

    overlaps.len() as i32
}
