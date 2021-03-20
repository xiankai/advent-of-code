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
    cid: Option<String>, // (Country ID)
}

fn validate_year_range(input: &str, min: i32, max: i32) -> Option<i32> {
    if input.len() != 4 { return None }
    let numeric_input = input.parse::<i32>().unwrap_or(0);
    if numeric_input >= min && numeric_input <= max {
        Some(numeric_input)
    } else {
        None
    }
}
fn validate_height(input: &str) -> Option<String> {
    let value = input[..input.len()-2].parse::<i32>();
    let unit = &input[input.len()-2..];
    match unit {
        "cm" => match value {
            Ok(150..=193) => Some(input.to_string()),
            _ => None,
        },
        "in" => match value {
            Ok(59..=76) => Some(input.to_string()),
            _ => None,
        },
        _ => None,
    }
}
fn validate_hair_color(input: &str) -> Option<String> {
    if !input.starts_with('#') { return None };
    if input.len() != 7 { return None };
    let mut invalid = false;
    for i in 1..7 {
        match &input.chars().nth(i).unwrap() {
            'a'..='f' | '0'..='9' => {},
            _ => { invalid = true },
        }
    }
    if invalid { None } else { Some(input.to_string()) }
}
fn validate_eye_color(input: &str) -> Option<String> {
    match input {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(input.to_string()),
        _ => None
    }
}
fn validate_passport_id(input: &str) -> Option<String> {
    if input.len() != 9 { return None }
    let mut invalid = false;
    input.chars().for_each(|x| if !x.is_numeric() { invalid = true });
    // println!("{} is {}", input, !invalid);
    if invalid { None } else { Some(input.to_string()) }
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
            let input = &entry[4..];
            match &entry[..3] {
                "byr" => passport.byr = validate_year_range(input, 1920, 2002),
                "iyr" => passport.iyr = validate_year_range(input, 2010, 2020),
                "eyr" => passport.eyr = validate_year_range(input, 2020, 2030),
                "hgt" => passport.hgt = validate_height(input),
                "hcl" => passport.hcl = validate_hair_color(input),
                "ecl" => passport.ecl = validate_eye_color(input),
                "pid" => passport.pid = validate_passport_id(input),
                "cid" => passport.cid = Some(input.to_string()),
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
        // println!("{:?}", passport);
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