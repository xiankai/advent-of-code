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

    let mut corrupted_number = 0;
    let mut max_end = 0;
    for i in 25..v.len()-25 {
        let slice = &v[i-25..i];

        let mut corrupted = true;
        for j in 0..slice.len() {
            for k in 1..slice.len() {
                if j == k {
                    continue;
                }
                if slice[j] + slice[k] == v[i] {
                    corrupted = false;
                }
            }
        }
        if corrupted {
            max_end = i;
            corrupted_number = v[i];
            break;
        }
    }
    println!("{} = corrupted at {}", corrupted_number, max_end);

    // let mut contiguous_sum = v[0] + v[1];
    // for start in 0..v.len() {
    //     for end in 1..v.len() {
    //         if contiguous_sum == corrupted_number {
    //             println!("encryption weakness: {} + {} = {}", v[start], v[end], v[start] + v[end]);
    //             return;
    //         }
    //         contiguous_sum += v[end];
    //     }
    //     contiguous_sum -= v[start];
    // }

    let mut start = 0;
    let mut end = 1;
    let mut contiguous_sum = v[start] + v[end];
    while contiguous_sum != corrupted_number {
        end += 1;
        contiguous_sum += v[end];
        if contiguous_sum > corrupted_number {
            start += 1;
            end = start + 1;
            contiguous_sum = v[start] + v[end];
        }
    }

    let slice = v[start..end].iter();
    let min = slice.clone().min().unwrap();
    let max = slice.max().unwrap();

    println!("encryption weakness: {} + {} = {}", min, max, min + max);
}