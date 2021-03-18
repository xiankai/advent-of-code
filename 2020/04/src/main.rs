use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<i32>, // (Birth Year)
    iyr: Option<i32>, // (Issue Year)
    eyr: Option<i32>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<i32>, // (Country ID)
}

fn read_input(path: &str) -> Result<Vec<Passport>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = vec![];

    let empty_passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };

    let mut passport = empty_passport.clone();
    for line in br.lines() {
        let y = line?;
        if y.is_empty() {
            v.push(passport);
            passport = empty_passport.clone();
            continue;
        }

        y.split(' ').into_iter().for_each(|entry| {
            let input = &entry[5..];
            let numeric_input = input.parse::<i32>();
            let string_input = input.to_string();
            match &entry[..3] {
                "byr" => passport.byr = Some(entry[5..].parse::<i32>().unwrap()),
                "iyr" => passport.iyr = Some(entry[5..].parse::<i32>().unwrap()),
                "eyr" => passport.eyr = Some(entry[5..].parse::<i32>().unwrap()),
                "hgt" => passport.hgt = Some(entry[5..].to_string()),
                "hcl" => passport.hcl = Some(entry[5..].to_string()),
                "ecl" => passport.ecl = Some(entry[5..].to_string()),
                "pid" => passport.pid = Some(entry[5..].to_string()),
                "cid" => passport.cid = Some(entry[4..].parse::<i32>().unwrap()),
                _ => println!(":("),
            }
        });
    }
    Ok(v)
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");

    let mut valid = 0;
    for passport in v.iter() {
        let mut invalid = false;
        if passport.byr.is_none() { invalid = true }
        if passport.iyr.is_none() { invalid = true }
        if passport.eyr.is_none() { invalid = true }
        if passport.hgt.is_none() { invalid = true }
        if passport.hcl.is_none() { invalid = true }
        if passport.ecl.is_none() { invalid = true }
        if passport.pid.is_none() { invalid = true }
        // if passport.cid.is_none() { invalid = true }
        if !invalid {
            valid += 1;
        }
    }
    println!("valid passports: {}", valid);
}