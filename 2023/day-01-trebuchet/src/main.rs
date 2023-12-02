use common::read_input_as_char_vectors;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);
    let char_vectors = read_input_as_char_vectors(br).unwrap();

    let sum = char_vectors_sum(&char_vectors);

    println!("{}", sum);

    let sum2 = char_vectors_sum_alphanumeric(&char_vectors);

    println!("{}", sum2);

    let sum3 = char_vectors_sum_alphanumeric_overlap(&char_vectors);

    println!("{}", sum3);
}

fn char_vectors_sum(char_vectors: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;

    for line in char_vectors {
        let numbers: Vec<i64> = line
            .iter()
            .filter_map(|c| c.to_string().parse::<i64>().ok())
            .collect();
        sum += numbers[0] * 10 + numbers[numbers.len() - 1]
    }

    sum
}

fn char_vectors_sum_alphanumeric(char_vectors: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    let mut buffer = String::from("");
    for line in char_vectors {
        let mut numbers: Vec<i64> = vec![];
        for chr in line {
            // found a number, add it and reset buffer
            if chr.is_numeric() {
                let number = chr.to_string().parse::<i64>().unwrap();
                numbers.push(number);
                buffer = String::from("");
            } else {
                // otherwise add to current buffer and trie it
                buffer.push(*chr);
                if let Some(number) = trie(&buffer) {
                    numbers.push(number);
                    buffer = String::from("");
                };
            }
        }

        // finished parsing line, let's get first and last number
        sum += numbers[0] * 10 + numbers[numbers.len() - 1]
    }

    sum
}

fn char_vectors_sum_alphanumeric_overlap(char_vectors: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for line in char_vectors {
        let numbers = extract_all_numbers(&line.iter().collect::<String>());
        sum += numbers[0] * 10 + numbers[numbers.len() - 1]
    }
    sum
}

fn trie(buffer: &str) -> Option<i64> {
    let number = if buffer.contains("one") {
        1
    } else if buffer.contains("two") {
        2
    } else if buffer.contains("three") {
        3
    } else if buffer.contains("four") {
        4
    } else if buffer.contains("five") {
        5
    } else if buffer.contains("six") {
        6
    } else if buffer.contains("seven") {
        7
    } else if buffer.contains("eight") {
        8
    } else if buffer.contains("nine") {
        9
    } else {
        0
    };

    if number == 0 {
        None
    } else {
        Some(number)
    }
}

fn extract_all_numbers(buffer: &str) -> Vec<i64> {
    let mut pairs = vec![];
    for (value, pattern) in [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (1, "1"),
        (2, "2"),
        (3, "3"),
        (4, "4"),
        (5, "5"),
        (6, "6"),
        (7, "7"),
        (8, "8"),
        (9, "9"),
    ] {
        for a_match in match_indices(buffer, pattern) {
            pairs.push((a_match, value));
        }
    }

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    pairs.iter().map(|(_, value)| *value).collect()
}

fn match_indices(buffer: &str, pattern: &str) -> Vec<usize> {
    buffer.match_indices(pattern).map(|(idx, _)| idx).collect()
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_char_vectors_sum() {
        let char_vectors = vec![
            vec!['1', '2'],
            vec!['3', '8'],
            vec!['1', '2', '3', '4', '5'],
            vec!['7'],
        ];

        let sum = char_vectors_sum(&char_vectors);

        assert_eq!(sum, 142);
    }

    #[test]
    fn test_char_vectors_sum_alphanumeric() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;

        let char_vectors = read_input_as_char_vectors(Cursor::new(input)).unwrap();

        let sum = char_vectors_sum_alphanumeric(&char_vectors);

        assert_eq!(sum, 281);
    }

    #[test]
    fn test_extract_all_numbers() {
        assert_eq!(extract_all_numbers("two1nine"), vec![2, 1, 9]);
        assert_eq!(extract_all_numbers("eightwothree"), vec![8, 2, 3]);
        assert_eq!(extract_all_numbers("abcone2threexyz"), vec![1, 2, 3]);
        assert_eq!(extract_all_numbers("xtwone3four"), vec![2, 1, 3, 4]);
        assert_eq!(extract_all_numbers("4nineeightseven2"), vec![4, 9, 8, 7, 2]);
        assert_eq!(extract_all_numbers("zoneight234"), vec![1, 8, 2, 3, 4]);
        assert_eq!(extract_all_numbers("7pqrstsixteen"), vec![7, 6]);
    }
}
