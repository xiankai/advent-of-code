use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn read_input(path: &str) -> Result<Vec<(char, i32)>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];
    for line in br.lines() {
        let y = line?;
        let (action, value) = y.split_at(1);
        v.push((action.chars().next().unwrap(), value.parse::<i32>().unwrap()))
    }
    Ok(v)
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");

    let mut x_offset = 0;
    let mut y_offset = 0;
    let mut x_delta = 10;
    let mut y_delta = 1;

    for (instruction, value) in v.iter() {
        match instruction {
            'N' => { y_delta += value },
            'S' => { y_delta -= value },
            'E' => { x_delta += value },
            'W' => { x_delta -= value },
            'L' => {
                let sin = f64::from(*value).to_radians().sin() as i32;
                let cos = f64::from(*value).to_radians().cos() as i32;
                // taken from https://stackoverflow.com/a/3162731
                let new_x_delta = x_delta * cos - y_delta * sin;
                y_delta = x_delta * sin + y_delta * cos;
                x_delta = new_x_delta;
             },
            'R' => {
                let sin = f64::from(*value).to_radians().sin() as i32;
                let cos = f64::from(*value).to_radians().cos() as i32;
                // taken from https://stackoverflow.com/a/3162731
                let new_x_delta = x_delta * cos + y_delta * sin;
                y_delta = -x_delta * sin + y_delta * cos;
                x_delta = new_x_delta;
            },
            'F' => {
                x_offset += x_delta * value;
                y_offset += y_delta * value;
            },
            _ => {},
        }
        // println!("waypoint = {},{}", x_delta,y_delta);
        // println!("x,y = {},{}", x_offset,y_offset);
    }
    println!("manhattan distance: {}", x_offset.abs() + y_offset.abs());
}