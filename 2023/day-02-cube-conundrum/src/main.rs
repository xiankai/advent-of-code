use common::read_input_as_string;

use regex::Regex;
use std::cmp::max;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);
    let strings = read_input_as_string(br).unwrap();

    let games = strings
        .iter()
        .map(|line: &String| parse_line_input(line))
        .collect::<Vec<Game>>();
    let id_sum = id_sum(&games);
    println!("{}", id_sum);

    let power = power(&games);
    println!("{}", power);
}

impl Game {
    fn is_possible(&self) -> bool {
        self.max_red <= 12 && self.max_blue <= 14 && self.max_green <= 13
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.max_red == other.max_red
            && self.max_blue == other.max_blue
            && self.max_green == other.max_green
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: 0,
            max_red: 0,
            max_blue: 0,
            max_green: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: i64,
    max_red: i64,
    max_blue: i64,
    max_green: i64,
}

impl Default for RevealSet {
    fn default() -> Self {
        RevealSet {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

#[derive(Debug)]
struct RevealSet {
    red: i64,
    blue: i64,
    green: i64,
}

fn parse_line_input(line: &str) -> Game {
    let mut game = Game {
        ..Default::default()
    };

    // ID
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let caps = re.captures(line).unwrap();
    let id = caps[1].parse::<i64>().unwrap();
    game.id = id;

    // Reveals
    caps[2].split(";").for_each(|reveal| {
        let mut reveal_set = RevealSet {
            ..Default::default()
        };
        reveal.split(",").for_each(|set| {
            let re = Regex::new(r"(\d+) (\w+)").unwrap();
            let caps = re.captures(set).unwrap();
            let value = caps[1].parse::<i64>().unwrap();
            match &caps[2] {
                "red" => {
                    reveal_set.red = value;
                    game.max_red = max(value, game.max_red)
                }
                "blue" => {
                    reveal_set.blue = value;
                    game.max_blue = max(value, game.max_blue)
                }
                "green" => {
                    reveal_set.green = value;
                    game.max_green = max(value, game.max_green)
                }
                _ => {}
            }
        });
    });

    game
}

fn id_sum(games: &Vec<Game>) -> i64 {
    let mut sum = 0;
    games.iter().for_each(|game| {
        if game.is_possible() {
            sum += game.id;
        }
    });
    sum
}

fn power(games: &Vec<Game>) -> i64 {
    let mut power = 0;
    games.iter().for_each(|game| {
        power += game.max_red * game.max_blue * game.max_green;
    });
    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_input() {
        assert_eq!(
            parse_line_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                max_red: 4,
                max_blue: 6,
                max_green: 2,
            }
        );
        assert_eq!(
            parse_line_input("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Game {
                id: 2,
                max_red: 1,
                max_blue: 4,
                max_green: 3,
            }
        );
        assert_eq!(
            parse_line_input(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            Game {
                id: 3,
                max_red: 20,
                max_blue: 6,
                max_green: 13,
            }
        );
        assert_eq!(
            parse_line_input(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            Game {
                id: 4,
                max_red: 14,
                max_blue: 15,
                max_green: 3,
            }
        );
        assert_eq!(
            parse_line_input("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Game {
                id: 5,
                max_red: 6,
                max_blue: 2,
                max_green: 3,
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_id_sum() {
        let games = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|line| parse_line_input(line))
        .collect();

        assert_eq!(id_sum(&games), 8);
    }

    #[test]
    fn test_power() {
        let games = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|line| parse_line_input(line))
        .collect();

        assert_eq!(power(&games), 2286);
    }
}
