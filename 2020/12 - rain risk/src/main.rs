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
    let mut direction = 0;

    for (instruction, value) in v.iter() {
        match instruction {
            'N' => { y_offset += value },
            'S' => { y_offset -= value },
            'E' => { x_offset += value },
            'W' => { x_offset -= value },
            'L' => { direction += value },
            'R' => { direction -= value },
            'F' => {
                x_offset += value * f64::from(direction).to_radians().cos() as i32;
                y_offset += value * f64::from(direction).to_radians().sin() as i32;
            },
            _ => {},
        }
        // println!("x,y at {}o = {},{}", direction,x_offset,y_offset);
    }
    println!("manhattan distance: {}", x_offset.abs() + y_offset.abs());
}