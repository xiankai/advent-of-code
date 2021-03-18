use std::fs::File;
use std::io::{BufReader, BufRead, Error};

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn read_input(path: &str) -> Result<Vec<Password>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];
    for line in br.lines() {
        let y = line?;
        let x: Vec<&str> = y.split(&[' ', '-', ':'][..]).collect();
        // let mut iter = line?.split(|x| x == ' ' || x == '-');

        let min = x[0].parse::<usize>().unwrap();
        let max = x[1].parse::<usize>().unwrap();
        let letter = x[2].parse::<char>().unwrap();
        let password = x[4].parse::<String>().unwrap();

        v.push(Password {
            min,
            max,
            letter,
            password,
        })
    }
    Ok(v)
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");
    let mut correct = 0;
    let mut new_correct = 0;
    for password in v.iter() {
        let instances = password.password.matches(password.letter).count();
        if instances >= password.min && instances <= password.max {
            correct += 1;
        }
        let mut new_instances = 0;
        if password.password.chars().nth(password.min - 1).unwrap() == password.letter {
            new_instances += 1;
        }
        if password.password.chars().nth(password.max - 1).unwrap() == password.letter {
            new_instances += 1;
        }
        if new_instances == 1 {
            new_correct += 1;
        }
    }
    println!("correct according to old policy: {}", correct);
    println!("correct according to new policy: {}", new_correct);
}