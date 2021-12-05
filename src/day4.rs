use std::iter;

use itertools::Itertools;

#[derive(Clone)]
pub struct Board {
    rows: Vec<Vec<i32>>,
    columns: Vec<Vec<i32>>
}

pub struct BingoData {
    seq: Vec<i32>,
    boards: Vec<Board>
}

#[aoc_generator(day4)]
pub fn day4_generator(input: &str) -> BingoData {
    let seq: Vec<i32> = input.lines().next().unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    for chunk in &input.lines().skip(1).filter(|l| !l.is_empty()).chunks(5) {
        let mut columns: Vec<Vec<i32>> = iter::repeat_with(|| Vec::new()).take(5).collect();
        let mut rows = Vec::new();

        chunk.for_each(|str_row| {
            let row: Vec<i32> = str_row.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            columns.iter_mut().zip(row.iter())
                .for_each(|(col, num)| col.push(*num));

            rows.push(row);
        });

        boards.push(Board { rows, columns });
    }

    BingoData {
        seq,
        boards
    }
}

#[aoc(day4, part1)]
pub fn day4_part1(data: &BingoData) -> i32 {
    println!("{:?}", data.boards.get(0).unwrap().rows);
    println!("{:?}", data.boards.get(0).unwrap().columns);

    let mut seq_so_far = Vec::new();
    let mut ans = 0;
    'seq: for i in 0..data.seq.len() {
        seq_so_far.push(data.seq.get(i).unwrap());

        for board in &data.boards {
            let valid_rows = board.rows.iter()
                .filter(|row| row.iter().filter(|n| seq_so_far.contains(n)).count() == 5).count();
            let valid_cols = board.columns.iter()
                .filter(|row| row.iter().filter(|n| seq_so_far.contains(n)).count() == 5).count();
            
            if valid_cols > 0 || valid_rows > 0 {
                let total_uncalled = board.rows.iter()
                    .fold(0, |acc, row| acc + row.iter()
                          .fold(0, |acc, n| if !seq_so_far.contains(&n) { acc + n } else { acc })
                         );

                //println!("Answer: {}", total_uncalled);
                //println!("Answer: {}", seq_so_far.pop().unwrap());
                ans = total_uncalled * seq_so_far.pop().unwrap();
                break 'seq;
            }
        }
    };

    ans
}

#[aoc(day4, part2)]
pub fn day4_part2(data: &BingoData) -> i32 {
    let mut boards_remaining = data.boards.clone();

    let mut seq_so_far = Vec::new();
    let mut ans = 0;
    'seq2: for i in 0..data.seq.len() {
        seq_so_far.push(data.seq.get(i).unwrap());

        let mut f = || {
            let mut board_to_be_removed: i32 = -1;
            for (i, board) in boards_remaining.iter().enumerate() {
                let valid_rows = board.rows.iter()
                    .filter(|row| row.iter().filter(|n| seq_so_far.contains(n)).count() == 5).count();
                let valid_cols = board.columns.iter()
                    .filter(|row| row.iter().filter(|n| seq_so_far.contains(n)).count() == 5).count();
                
                if valid_cols > 0 || valid_rows > 0 {
                    if boards_remaining.len() > 1 {
                        board_to_be_removed = i as i32;
                        break;
                    }
                    let total_uncalled = board.rows.iter()
                        .fold(0, |acc, row| acc + row.iter()
                              .fold(0, |acc, n| if !seq_so_far.contains(&n) { acc + n } else { acc })
                             );

                    //println!("Answer: {}", total_uncalled);
                    //println!("Answer: {}", seq_so_far.pop().unwrap());
                    ans = total_uncalled * seq_so_far.pop().unwrap();
                    return -2;
                }
            }
            if board_to_be_removed > -1 {
                boards_remaining.remove(board_to_be_removed as usize);
                return -1;
            }
            
            return 0;
        };

        // Filthy
        loop {
            let result = f();
            if result == -2 {
                break 'seq2;
            } else if result == 0 {
                break;
            }
        }
    };

    ans
}
