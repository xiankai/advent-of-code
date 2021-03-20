use std::fs::File;
use std::io::{BufReader, BufRead, Error};

struct Instruction {
    operation: String,
    value: i32,
    times_ran: i32,
}

fn read_input(path: &str) -> Result<Vec<Instruction>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut instructions = vec![];
    for line in br.lines() {
        let y = line?;
        instructions.push(Instruction {
            operation: y[..3].to_string(),
            value: y[4..].parse::<i32>().unwrap(),
            times_ran: 0,
        })
    }
    Ok(instructions)
}

fn main() {
    let mut instructions = read_input("./input.txt").expect("invalid input");

    let mut accumulator = 0;
    let mut index = 0;
    let mut terminated = false;
    while !terminated {
        let current_instruction = &instructions[index];

        if current_instruction.times_ran > 0 {
            terminated = true;
            continue;
        }

        let prev_index = index;
        let value = current_instruction.value;
        let operation = &current_instruction.operation[..];

        match operation {
            "acc" => {
                accumulator += value;
                index += 1;
            },
            "jmp" => {
                if value.is_negative() { index -= value.abs() as usize } else { index += value as usize }
            },
            "nop" => index += 1,
            _ => panic!("unfound instruction {}", current_instruction.operation),
        }
        instructions[prev_index].times_ran += 1;

        if index == instructions.len() {
            terminated = true;
        }
    }

    println!("{}", accumulator);
}