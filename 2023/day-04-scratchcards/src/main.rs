use common::read_input_as_string;

use std::fs::File;
use std::io::BufReader;

use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);

    let input = read_input_as_string(br).unwrap();
    let scratchcards = parse_into_scratchcards(input);

    let points = scratchcards.iter().map(|scratchcard| get_points(&scratchcard)).sum::<i64>();
    println!("Total points: {}", points);

    let scratchcard_count = get_moar_wins(scratchcards);
    println!("Total scratchcards: {}", scratchcard_count);
}

fn parse_into_scratchcards(scratchcards: Vec<String>) -> Vec<(Vec<i64>, Vec<i64>)> {
    scratchcards.into_iter().map(|scratchcard| {
        let mut temp = scratchcard.split('|');
        let winning_numbers = temp.next().unwrap().split(':').last().unwrap();
        let own_numbers = temp.next().unwrap();

        let re: Regex = Regex::new(r"(\d+)").unwrap();
        (
            re.find_iter(winning_numbers).map(|number| number.as_str().parse::<i64>().unwrap()).collect(),
            re.find_iter(own_numbers).map(|number| number.as_str().parse::<i64>().unwrap()).collect(),
        )
    }).collect()
}

fn get_points(scratchcard: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut point_value = 1;
    for own_number in &scratchcard.1 {
        if scratchcard.0.contains(&own_number) {
            point_value *= 2;
        }
    }
    if point_value == 1 { 0 } else { point_value / 2 }
}

fn get_wins(scratchcard: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut wins = 0;
    for own_number in &scratchcard.1 {
        if scratchcard.0.contains(&own_number) {
            wins += 1;
        }
    }
    wins
}

fn get_moar_wins(scratchcards: Vec<(Vec<i64>, Vec<i64>)>) -> usize {
    let mut win_moar = hashbrown::HashMap::new();
    for (n, scratchcard) in scratchcards.iter().enumerate() {
        let small_wins = get_wins(scratchcard) as usize;
        let previous_wins = *win_moar.get(&n).unwrap_or(&0) + 1;
        for i in n+1..n+1+small_wins {
            win_moar.entry(i).and_modify(|e| *e += previous_wins).or_insert(previous_wins);
        }
        // println!("{} small_wins, {} previous wins, {} total wins, state {:?}", small_wins, previous_wins, wins, win_moar);
    }

    win_moar.values().sum::<usize>() + scratchcards.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;


    #[test]
    fn test_get_points() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;

        let input = read_input_as_string(Cursor::new(input)).unwrap();
        let scratchcards = parse_into_scratchcards(input);

        assert_eq!(get_points(&scratchcards[0]), 8);
        assert_eq!(get_points(&scratchcards[1]), 2);
        assert_eq!(get_points(&scratchcards[2]), 2);
        assert_eq!(get_points(&scratchcards[3]), 1);
        assert_eq!(get_points(&scratchcards[4]), 0);
        assert_eq!(get_points(&scratchcards[5]), 0);
    }

    #[test]
    fn test_get_wins() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;

        let input = read_input_as_string(Cursor::new(input)).unwrap();
        let scratchcards = parse_into_scratchcards(input);

        assert_eq!(get_wins(&scratchcards[0]), 4);
        assert_eq!(get_wins(&scratchcards[1]), 2);
        assert_eq!(get_wins(&scratchcards[2]), 2);
        assert_eq!(get_wins(&scratchcards[3]), 1);
        assert_eq!(get_wins(&scratchcards[4]), 0);
        assert_eq!(get_wins(&scratchcards[5]), 0);
    }

    #[test]
    fn test_get_moar_wins() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;

        let input = read_input_as_string(Cursor::new(input)).unwrap();
        let scratchcards = parse_into_scratchcards(input);

        assert_eq!(get_moar_wins(scratchcards), 30);
    }
}