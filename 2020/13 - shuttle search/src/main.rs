use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_input(path: &str) -> (i32, Vec<i32>) {
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    let mut iter = br.lines();
    let earliest_timestamp = iter.next().unwrap().unwrap().parse::<i32>().unwrap();
    let schedule: Vec<i32> = iter.next().unwrap().unwrap().split(',').filter(|x| x != &"x").map(|x| x.parse::<i32>().unwrap()).collect();
    (earliest_timestamp, schedule)
}

fn main() {
    let (earliest_timestamp, schedule) = read_input("./input.txt");

    let mut earliest_bus = 0;
    let mut earliest_time_waited = earliest_timestamp;
    for bus in schedule.iter() {
        let time_waited = bus - earliest_timestamp % bus;
        if time_waited < earliest_time_waited {
            earliest_time_waited = time_waited;
            earliest_bus = *bus;
        }
    }
    println!("{} * {} = {}", earliest_bus, earliest_time_waited, earliest_bus * earliest_time_waited);
}