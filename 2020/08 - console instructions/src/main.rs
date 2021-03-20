use std::fs::File;
use std::io::{BufReader, BufRead, Error};

struct Instruction {
    operation: String,
    value: i32,
    tried_changing: bool,
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
            tried_changing: false,
        })
    }
    Ok(instructions)
}

fn main() {
    let mut instructions = read_input("./input.txt").expect("invalid input");

    let mut accumulator = 0;
    let mut index = 0;
    let mut has_tried_changing = false;
    let mut instructions_executed = vec![];

    let mut terminated = false;
    while !terminated {
        let current_instruction = &instructions[index];

        if instructions_executed.contains(&index) {
            instructions_executed = vec![];
            accumulator = 0;
            index = 0;
            has_tried_changing = false;
            // terminated = true;
            continue;
        }

        let prev_index = index;
        let value = current_instruction.value;
        let mut operation = &current_instruction.operation[..];
        let mut tried_changing = false;

        // println!("did we try yet? {}, has {} been tried yet? {}", has_tried_changing, prev_index, current_instruction.tried_changing);
        if !has_tried_changing && !current_instruction.tried_changing {
            match operation {
                "jmp" => operation = "nop",
                "nop" => operation = "jmp",
                _ => {},
            }
            tried_changing = true;
        }

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

        instructions_executed.push(prev_index);
        if tried_changing {
            // println!("tried {}", prev_index);
            instructions[prev_index].tried_changing = true;
            has_tried_changing = true;
        }

        if index == instructions.len() {
            terminated = true;
        }
    }

    println!("{}", accumulator);
}