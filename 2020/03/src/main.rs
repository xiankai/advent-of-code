use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn read_input(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?)
    }
    Ok(v)
}

fn traverse_slope(terrain: &[String], x_movement: usize, y_movement: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let len_x = terrain[0].len();
    let max_y = terrain.len() - 1;

    let mut end = false;
    let mut trees = 0;
    while !end {
        // println!("({},{}) {}", y, x, terrain[y].chars().nth(x%len_x).unwrap());
        if terrain[y].chars().nth(x%len_x).unwrap() == '#' {
            trees += 1;
        }

        if y < max_y {
            x += x_movement;
            y += y_movement;
        } else {
            end = true;
        }
    }

    trees
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");

    let slope_1_1 = traverse_slope(&v, 1, 1);
    let slope_3_1 = traverse_slope(&v, 3, 1);
    let slope_5_1 = traverse_slope(&v, 5, 1);
    let slope_7_1 = traverse_slope(&v, 7, 1);
    let slope_1_2 = traverse_slope(&v, 1, 2);

    println!("{}*{}*{}*{}*{}={}", slope_1_1, slope_3_1, slope_5_1, slope_7_1, slope_1_2, slope_1_1 * slope_3_1 * slope_5_1 * slope_7_1 * slope_1_2);
}