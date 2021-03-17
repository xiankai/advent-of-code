use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn read_input(path: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?)
    }
    Ok(v)
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");
    let len = v.len();
    for i in 0..len {
        for j in i+1..len {
            if v[i] + v[j] == 2020 {
                println!("{} * {} = {}", v[i], v[j], v[i] * v[j]);
            }
            for k in i+2..len {
                if v[i] + v[j] + v[k] == 2020 {
                    println!("{} * {} * {} = {}", v[i], v[j], v[k], v[i] * v[j] * v[k]);
                }
            }
        }
    }
}