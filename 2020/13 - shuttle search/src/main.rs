use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_input(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    let mut iter = br.lines();
    let _earliest_timestamp = iter.next().unwrap().unwrap().parse::<i64>().unwrap();
    let schedule = iter.next().unwrap().unwrap().split(',').map(|x| x.to_string()).collect();
    schedule
}

fn main() {
    let schedule = read_input("./input.txt");

    let mut earliest_timestamp: i64 = 0;
    let mut buses: Vec<(i64, i64)> = vec!();
    let mut biggest_bus = 0;
    for (i, bus) in schedule.iter().enumerate() {
        if bus == "x" { continue; }
        let bus_id = bus.parse::<i64>().unwrap();
        buses.push((i as i64, bus_id));
        if bus_id > biggest_bus {
            biggest_bus = bus_id;
            earliest_timestamp = -(i as i64);
        }
    };

    println!("{:?}", buses);
    earliest_timestamp += 100000000000000 + (100000000000000 % biggest_bus);

    while buses.iter().any(|(i, bus_id)| (earliest_timestamp + i) % bus_id != 0) {
        earliest_timestamp += biggest_bus;
        // println!("{}", earliest_timestamp);
        // break;
    }

    println!("{}", earliest_timestamp);
}