use common::read_input_as_string;

use std::fs::File;
use std::io::BufReader;

use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);

    let input = read_input_as_string(br).unwrap();

    let regex = Regex::new(r"(\d+)").unwrap();
    let times = regex.captures_iter(&input[0]).map(|cap| cap[1].parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let distances = regex.captures_iter(&input[1]).map(|cap| cap[1].parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let new_regex = Regex::new(r"(\d)").unwrap();
    let combined_times = new_regex.captures_iter(&input[0]).map(|cap| cap[1].parse::<char>().unwrap()).collect::<Vec<char>>().iter().collect::<String>().parse::<i64>().unwrap();
    let combined_distances = new_regex.captures_iter(&input[1]).map(|cap| cap[1].parse::<char>().unwrap()).collect::<Vec<char>>().iter().collect::<String>().parse::<i64>().unwrap();

    println!("{}", the_answers(times, distances));
    println!("{}", the_answer(combined_times, combined_distances));
}

fn the_answer(time: i64, distance: i64) -> i64 {
    let mut methods = 0;
    for time_taken in 0..time {
        if time_taken * (time - time_taken) > distance {
            methods += 1;
        }
    }
    methods
}

fn the_answers(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let mut margin = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        margin *= the_answer(*time, *distance);
    }
    margin
}