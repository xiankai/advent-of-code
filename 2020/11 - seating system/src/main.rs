use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn read_input(path: &str) -> Result<SeatMap, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?.chars().collect())
    }
    Ok(v)
}

type SeatRow = Vec<char>;
type SeatMap = Vec<SeatRow>;

fn check_row(row: &SeatRow, middle_column: usize) -> i32 {
    let mut occupied = 0;
    // checking left
    if middle_column > 0 && row[middle_column - 1] == '#' {
        occupied += 1
    };
    // checking right
    if middle_column < row.len() - 1 && row[middle_column + 1] == '#' {
        occupied += 1
    };
    if row[middle_column] == '#' {
        occupied += 1;
    }
    occupied
}

fn fill_seats(seat_map: SeatMap) -> (SeatMap, i32, i32) {
    let mut new_seat_map = seat_map.clone();
    let mut now_occupied = 0;
    let mut now_empty = 0;
    for (i, row) in seat_map.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            let mut seat_count = 0;
            if i > 0 { seat_count += check_row(&seat_map[i - 1], j) } // above
            if i < seat_map.len() - 1 { seat_count += check_row(&seat_map[i + 1], j) } // below
            seat_count += check_row(row, j);
            // println!("({},{}) has {} friends", i+1, j+1, seat_count);
            match seat {
                '#' => { // occupied seat
                    if seat_count >= 5 { // -1 because the middle seat itself is already occupied
                        new_seat_map[i][j] = 'L';
                        now_empty += 1;
                    }
                },
                'L' => { // empty seat
                    if seat_count == 0 {
                        new_seat_map[i][j] = '#';
                        now_occupied += 1;
                    }
                },
                _ => {},
            }
        }
    }
    (new_seat_map, now_empty, now_occupied)
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");

    let mut seat_map = v;
    let mut iteration = 0;
    let mut changed = true;
    while changed {
        let (new_seat_map, now_empty, now_occupied) = fill_seats(seat_map);
        seat_map = new_seat_map;
        changed = now_empty + now_occupied > 0;
        iteration += 1;
        // println!("#{}, emptied: {}, occupied: {}", iteration, now_empty, now_occupied);
    }

    let occupied_seats: usize = seat_map
        .iter()
        .map(|row|
            row
            .iter()
            .map(|seat|
                if seat == &'#' { 1 } else { 0 }
            ).sum::<usize>()
        )
        .sum();
    println!("occupied seats: {}", occupied_seats);
}