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

#[allow(dead_code)]
fn check_row(row: &SeatRow, middle_column: usize) -> i32 {
    let mut occupied = 0;
    // checking left
    if middle_column > 0 && row[..middle_column - 1].iter().any(|x| x == &'#') {
        occupied += 1
    };
    // checking right
    if middle_column < row.len() - 1 && row[middle_column + 1..].iter().any(|x| x == &'#') {
        occupied += 1
    };
    if row[middle_column] == '#' {
        occupied += 1;
    }
    occupied
}

fn traverse(seat_map: &SeatMap, middle_column: i32, middle_row: i32, delta_x: i32, delta_y: i32) -> i32 {
    let mut x = middle_column;
    let mut y = middle_row;

    // execute at least once
    x += delta_x;
    y += delta_y;

    while x >= 0 && x < seat_map[0].len() as i32 && y >= 0 && y < seat_map.len() as i32 {
        if seat_map[y as usize][x as usize] == '#' { return 1; }
        if seat_map[y as usize][x as usize] == 'L' { return 0; }
        x += delta_x;
        y += delta_y;
    }
    0
}

fn check_directions(seat_map: &SeatMap, middle_column: i32, middle_row: i32) -> i32 {
    let mut occupied = 0;

    occupied += traverse(seat_map, middle_column, middle_row, -1, -1);
    occupied += traverse(seat_map, middle_column, middle_row, -1, 0);
    occupied += traverse(seat_map, middle_column, middle_row, -1, 1);
    occupied += traverse(seat_map, middle_column, middle_row, 0, -1);
    occupied += traverse(seat_map, middle_column, middle_row, 0, 1);
    occupied += traverse(seat_map, middle_column, middle_row, 1, -1);
    occupied += traverse(seat_map, middle_column, middle_row, 1, 0);
    occupied += traverse(seat_map, middle_column, middle_row, 1, 1);

    occupied
}

fn fill_seats(seat_map: SeatMap) -> (SeatMap, i32, i32) {
    let mut new_seat_map = seat_map.clone();
    let mut now_occupied = 0;
    let mut now_empty = 0;
    for (i, row) in seat_map.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            let seat_count = check_directions(&seat_map, j as i32, i as i32);
            // let mut seat_count = 0;
            // if i > 0 { seat_count += check_row(&seat_map[i - 1], j) } // above
            // if i < seat_map.len() - 1 { seat_count += check_row(&seat_map[i + 1], j) } // below
            // seat_count += check_row(row, j);
            // println!("({},{}) has {} friends", i+1, j+1, seat_count);
            match seat {
                '#' => { // occupied seat
                    if seat_count >= 5 { // old method needs rule threshold + 1, when including the owner's seat
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
    // let mut iteration = 0;
    let mut changed = true;
    while changed {
        let (new_seat_map, now_empty, now_occupied) = fill_seats(seat_map);
        seat_map = new_seat_map;
        changed = now_empty + now_occupied > 0;
        // iteration += 1;
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
        // println!("seat map: \n{}", seat_map.into_iter().map(|x| x.into_iter().collect()).collect::<Vec<String>>().join("\n"));
}