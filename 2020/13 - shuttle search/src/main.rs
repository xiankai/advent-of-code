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

    let mut buses: Vec<[i64; 2]> = vec!();
    for (i, bus) in schedule.iter().enumerate() {
        if bus == "x" { continue; }
        let bus_id = bus.parse::<i64>().unwrap();
        buses.push([i as i64, bus_id]);
    };

    println!("{:?}", buses);
    // solution from https://old.reddit.com/r/adventofcode/comments/kcb3bb/2020_day_13_part_2_can_anyone_tell_my_why_this/
    let mut earliest_timestamp: i64 = 0;

    let mut lcm = buses[0][1];
    let mut bus_being_checked = buses[1][1];
    let mut offset = buses[1][0];
    let mut buses_to_check = 2;

    loop {
        if (earliest_timestamp + offset) % bus_being_checked == 0 {
            lcm *= bus_being_checked;
            buses_to_check += 1;
            if buses_to_check > buses.len() { break; }
            bus_being_checked = buses[buses_to_check - 1][1];
            offset = buses[buses_to_check - 1][0];
        }
        earliest_timestamp += lcm;
        // println!("{}", earliest_timestamp);
    }

    println!("{}", earliest_timestamp);
}