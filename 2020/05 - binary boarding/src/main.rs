use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn read_input(path: &str) -> Result<Vec<BoardPass>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];
    for line in br.lines() {
        v.push(decode_boarding_pass(&line?))
    }
    Ok(v)
}

struct BoardPass {
    code: String,
    row: i32,
    column: i32,
    seat_id: i32,
}

fn decode_boarding_pass(pass: &str) -> BoardPass {
    let mut lower_row = 0;
    let mut upper_row = 128;
    let mut lower_column = 0;
    let mut upper_column = 8;
    for ch in pass.chars().into_iter() {
        let row_space = (upper_row - lower_row) / 2;
        let column_space = (upper_column - lower_column) / 2;
        match ch {
            'F' => { upper_row -= row_space },
            'B' => { lower_row += row_space },
            'L' => { upper_column -= column_space },
            'R' => { lower_column += column_space },
            _ => {},
        }
    }
    BoardPass {
        code: pass.to_string(),
        row: lower_row,
        column: lower_column,
        seat_id: lower_row * 8 + lower_column,
    }
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");

    let mut lowest_seat_id = 128 * 8;
    let mut highest_seat_id = 0;
    let mut actual_seat_ids: Vec<i32> = vec![];
    for pass in v.iter() {
        if pass.seat_id > highest_seat_id {
            highest_seat_id = pass.seat_id;
        }

        if pass.seat_id < lowest_seat_id {
            lowest_seat_id = pass.seat_id;
        }

        actual_seat_ids.push(pass.seat_id);
    }

    actual_seat_ids.sort_unstable();
    let mut prev_seat_id = 0;
    for seat_id in actual_seat_ids.iter() {
        if seat_id - prev_seat_id == 2 {
            println!("My Seat ID: {}", seat_id - 1);
        }
        prev_seat_id = *seat_id;
    }
    println!("Highest Seat ID: {}", highest_seat_id);
    println!("Seats: {}", v.len());
}