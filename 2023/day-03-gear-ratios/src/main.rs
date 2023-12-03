use common::read_input_as_char_vectors;

use std::fs::File;
use std::io::BufReader;

use std::cmp::min;

use regex::Regex;

use hashbrown::HashMap;

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);
    let schematic = read_input_as_char_vectors(br).unwrap();

    let missing_part_number = get_missing_part_number(&schematic);
    println!("{}", missing_part_number);

    let gear_ratio_sum = get_gear_ratio_sum(&schematic);
    println!("{}", gear_ratio_sum);
}

fn identify_numbers_by_regex(schematic: &Vec<Vec<char>>) -> Vec<(usize, usize, usize, i64)> {
    let mut numbers = vec!();
    for (row_index, row) in schematic.iter().enumerate() {
        let line = row.iter().collect::<String>();
        for number in find_numbers_in_line(line) {
            numbers.push((row_index, number.0, number.1, number.2));
        }
    }
    numbers
}

fn find_numbers_in_line(line: String) -> Vec<(usize, usize, i64)> {
    let re: Regex = Regex::new(r"(\d+)").unwrap();
    let mut numbers = vec!();
    for cap in re.captures_iter(&line) {
        let start = cap.get(1).unwrap().start();
        let end = cap.get(1).unwrap().end() - 1;
        let value = cap[1].parse::<i64>().unwrap();
        numbers.push((start, end, value));
    }
    numbers
}

fn identify_numbers(schematic: &Vec<Vec<char>>) -> Vec<(usize, usize, usize, i64)> {
    let mut numbers = vec!();
    for (row_index, row) in schematic.iter().enumerate() {
        let mut number_buffer = vec!();
        let mut starting_column = 0;
        let mut ending_column = 0;
        let mut buffering_number = false;
        for (column, number) in row.iter().enumerate() {
            // not number
            if !number.is_numeric() {
                // flush number buffer if needed
                if buffering_number {
                    ending_column = column - 1;
                    numbers.push((
                        row_index,
                        starting_column,
                        ending_column,
                        number_buffer.clone().into_iter().collect::<String>().parse::<i64>().unwrap()
                    ));
                    // reset afterwards
                    buffering_number = false;
                    number_buffer.clear();
                }
                continue;
            }

            // subsequent number
            if buffering_number {
                number_buffer.push(number);
            } else {
                // first number
                buffering_number = true;
                starting_column = column;
                number_buffer.push(number);
            }
        }
        // if still buffering number at the end, flush it
        if buffering_number {
            numbers.push((
                row_index,
                starting_column,
                ending_column,
                number_buffer.clone().into_iter().collect::<String>().parse::<i64>().unwrap()
            ));
        }
    }
    numbers
}

fn get_search_space(schematic: &Vec<Vec<char>>, row: usize, start: usize, end: usize) -> Vec<(usize, usize)> {
    let row_len = schematic[row].len();
    let col_len = schematic.len();
    let min_col = if start > 0 { start - 1 } else { 0 };
    let max_col = min(col_len - 1, end + 1);

    let mut search_space: Vec<(usize, usize)> = vec!();

    // top
    if row > 0 {
        for col in min_col..=max_col {
            search_space.push((row - 1, col));
        }
    }

    // left
    if start > min_col {
        search_space.push((row, start - 1));
    }

    // right
    search_space.push((row, max_col));

    // bottom
    if row < row_len - 1 {
        for col in min_col..=max_col {
            search_space.push((row + 1, col));
        }
    }

    search_space
}

fn is_part_number(schematic: &Vec<Vec<char>>, row: usize, start: usize, end: usize) -> bool {
    let search_space = get_search_space(schematic, row, start, end);

    for (row, col) in search_space {
        if schematic[row][col] != '.' && !schematic[row][col].is_numeric() {
            return true;
        }
    }
    return false;
}

fn is_gear_number(schematic: &Vec<Vec<char>>, row: usize, start: usize, end: usize) -> Option<(usize, usize)> {
    let search_space = get_search_space(schematic, row, start, end);

    for (row, col) in search_space {
        if schematic[row][col] == '*' {
            return Some((row, col));
        }
    }
    return None;
}

fn get_missing_part_number(schematic: &Vec<Vec<char>>) -> i64 {
    // let numbers = identify_numbers(schematic);
    let numbers: Vec<(usize, usize, usize, i64)> = identify_numbers_by_regex(schematic);
    let mut sum = 0;
    for number in numbers {
        let (row, start, end, value) = number;
        if is_part_number(schematic, row, start, end) {
            sum += value;
        }
    }
    sum
}

fn get_gear_ratio_sum(schematic: &Vec<Vec<char>>) -> i64 {
    let numbers: Vec<(usize, usize, usize, i64)> = identify_numbers_by_regex(schematic);
    let mut gear_numbers: HashMap<String, Vec<i64>> = HashMap::new();
    for number in numbers {
        let (row, start, end, value) = number;
        if let Some(gear) = is_gear_number(schematic, row, start, end) {
            let key = format!("{},{}",gear.0, gear.1);
            let entry = gear_numbers.entry(key);
            entry.or_default().push(value);
        }
    }
    let sum = gear_numbers.iter().map(|(_, v)| {
        if v.len() == 2 {
            v.iter().product::<i64>()
        } else {
            0
        }
    }).sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_identify_numbers() {
        let raw_schematic = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        let schematic = read_input_as_char_vectors(Cursor::new(raw_schematic)).unwrap();

        assert_eq!(identify_numbers(&schematic), vec![
            (0, 0, 2, 467),
            (0, 5, 7, 114),
            (2, 2, 3, 35),
            (2, 6, 8, 633),
            (4, 0, 2, 617),
            (5, 7, 8, 58),
            (6, 2, 4, 592),
            (7, 6, 8, 755),
            (9, 1, 3, 664),
            (9, 5, 7, 598)
        ]);
    }

    #[test]
    fn test_is_part_number() {
        let raw_schematic = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        let schematic = read_input_as_char_vectors(Cursor::new(raw_schematic)).unwrap();

        assert_eq!(is_part_number(&schematic, 0, 0, 2), true);
        assert_eq!(is_part_number(&schematic, 0, 5, 7), false);
        assert_eq!(is_part_number(&schematic, 2, 2, 3), true);
        assert_eq!(is_part_number(&schematic, 2, 6, 8), true);
        assert_eq!(is_part_number(&schematic, 4, 0, 2), true);
        assert_eq!(is_part_number(&schematic, 5, 7, 8), false);
        assert_eq!(is_part_number(&schematic, 6, 2, 4), true);
        assert_eq!(is_part_number(&schematic, 7, 6, 8), true);
        assert_eq!(is_part_number(&schematic, 9, 1, 3), true);
        assert_eq!(is_part_number(&schematic, 9, 5, 7), true);
    }

    #[test]
    fn test_is_part_number_other_symbols() {
        let raw_schematic = r#"
            ...
            .3.
            ..&
        "#;
        let schematic = read_input_as_char_vectors(Cursor::new(raw_schematic)).unwrap();

        assert_eq!(is_part_number(&schematic, 1, 1, 3), true);
    }

    #[test]
    fn test_all_positions() {
        let raw_schematic = r#"
            5.6
            .&.
            4.3
        "#;
        let schematic = read_input_as_char_vectors(Cursor::new(raw_schematic)).unwrap();

        assert_eq!(is_part_number(&schematic, 2, 2, 3), true);
        assert_eq!(is_part_number(&schematic, 2, 0, 4), true);
        assert_eq!(is_part_number(&schematic, 0, 0, 5), true);
        assert_eq!(is_part_number(&schematic, 0, 2, 6), true);
    }

    #[test]
    fn test_get_missing_part_number() {
        let raw_schematic = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        let schematic = read_input_as_char_vectors(Cursor::new(raw_schematic)).unwrap();

        assert_eq!(get_missing_part_number(&schematic), 4361);
    }

    #[test]
    fn test_regex() {
        assert_eq!(find_numbers_in_line("467..114..".to_string()), vec![(0, 2, 467), (5, 7, 114)]);
    }

    #[test]
    fn test_get_gear_ratio_sum() {
        let raw_schematic = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        let schematic = read_input_as_char_vectors(Cursor::new(raw_schematic)).unwrap();

        assert_eq!(get_gear_ratio_sum(&schematic), 467835);
    }
}
