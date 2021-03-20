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

fn factorial(n: i64) -> i64 {
    (1..=n).product()
}

fn combination(n: i64, r: i64) -> i64 {
    factorial(n)/factorial(r)/factorial(n-r)
}

fn combination_sum(n: i64) -> i64 {
    let r = ((n/3) as f64).ceil() as i64;
    // println!("{}, {}", n, r);
    (r..=n).map(|r| combination(n, r)).sum()
}

fn main() {
    let mut v = read_input("./input.txt").expect("invalid input");

    v.sort_unstable();

    let mut prev = 0; // charging outlet
    let mut diff_1 = 0;
    let mut diff_3 = 1; // device
    let mut total_arrangements = 1;

    // for part 2
    let mut stretches_of_1 = 0;
    let mut prev_diff = 0;

    for i in v.iter() {
        // println!("{}", i);
        let diff = i - prev;
        match diff {
            1 => {
                diff_1 += 1;
                stretches_of_1 += 1;
            },
            3 => {
                diff_3 += 1;
                if prev_diff == 1 {
                    total_arrangements *= combination_sum(stretches_of_1 - 1);
                }
                stretches_of_1 = 0;
            },
            _ => {},
        }
        prev_diff = diff;
        prev = *i;
    }
    if prev_diff == 1 {
        total_arrangements *= combination_sum(stretches_of_1 - 1);
    }

    println!("1j * 3j = {}", diff_1 * diff_3);
    println!("total arrangements = {}", total_arrangements);
}