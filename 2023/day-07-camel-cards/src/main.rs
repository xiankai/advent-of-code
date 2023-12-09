use common::read_input_as_string;

use std::fs::File;
use std::io::BufReader;
use std::cmp::Ordering;

use regex::Regex;


fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);

    let input = read_input_as_string(br).unwrap();

    let regex = Regex::new(r"(.+) (\d+)").unwrap();
    let hands: Vec<Hand> = input.iter().map(|line| {
        let captures = regex.captures(line).unwrap();
        let cards = captures[1].parse::<String>().unwrap();
        let bid = captures[2].parse::<i64>().unwrap();
        Hand { cards: c_to_n(&cards), bid }
    }).collect();

    println!("{}", total_winnings(hands));

    let joker_hands: Vec<HandJoker> = input.iter().map(|line| {
        let captures = regex.captures(line).unwrap();
        let cards = captures[1].parse::<String>().unwrap();
        let bid = captures[2].parse::<i64>().unwrap();
        HandJoker { cards: c_to_n_with_j(&cards), bid }
    }).collect();
    println!("{}", total_winnings_with_jokers(joker_hands));
}

#[derive(Clone, Debug)]
struct Hand {
    cards: [i64;5],
    bid: i64,
}

#[derive(Clone, Debug)]
struct HandJoker {
    cards: [i64;5],
    bid: i64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if get_type(self) > get_type(other) {
            return Ordering::Greater;
        } else if get_type(self) < get_type(other) {
            return Ordering::Less;
        } else {
            if get_value_type(self) > get_value_type(other) {
                return Ordering::Greater;
            } else if get_value_type(self) < get_value_type(other) {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        get_abs_value(self) == get_abs_value(other)
    }
}

impl Eq for Hand {}

impl Ord for HandJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        if get_joker_type(self) > get_joker_type(other) {
            return Ordering::Greater;
        } else if get_joker_type(self) < get_joker_type(other) {
            return Ordering::Less;
        } else {
            if get_value_type_2(self) > get_value_type_2(other) {
                return Ordering::Greater;
            } else if get_value_type_2(self) < get_value_type_2(other) {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandJoker {
    fn eq(&self, other: &Self) -> bool {
        get_abs_value_2(self) == get_abs_value_2(other)
    }
}

impl Eq for HandJoker {}

#[derive(Eq, PartialOrd, PartialEq, Debug)]
enum Type {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn get_abs_value(hand: &Hand) -> i64 {
    get_type(hand) as i64 * 100_00_00_00_00 + get_value_type(hand)
}

fn get_abs_value_2(hand: &HandJoker) -> i64 {
    get_type_2(hand) as i64 * 100_00_00_00_00 + get_value_type_2(hand)
}

fn get_type(hand: &Hand) -> Type {
    let mut hand_type: Type = Type::HighCard;
    let mut set: [i64; 15] = [0; 15];
    for card in hand.cards.iter() {
        set[*card as usize] += 1;
        match set[*card as usize] {
            5 => { hand_type = Type::FiveOfAKind }
            4 => { hand_type = Type::FourOfAKind }
            3 => { hand_type = Type::ThreeOfAKind }
            _ => {  }
        }
    }

    let unique_cards = set.iter().filter(|v| v != &&0).count();
    if unique_cards == 2 && hand_type == Type::ThreeOfAKind {
        return Type::FullHouse;
    }

    let pairs = set.iter().filter(|v| v == &&2). count();
    match pairs {
        1 => { hand_type = Type::OnePair }
        2 => { hand_type = Type::TwoPairs }
        _ => { }
    }

    return hand_type;
}

fn get_type_2(hand: &HandJoker) -> Type {
    let mut hand_type: Type = Type::HighCard;
    let mut set: [i64; 15] = [0; 15];
    for card in hand.cards.iter() {
        set[*card as usize] += 1;
        match set[*card as usize] {
            5 => { hand_type = Type::FiveOfAKind }
            4 => { hand_type = Type::FourOfAKind }
            3 => { hand_type = Type::ThreeOfAKind }
            _ => {  }
        }
    }

    let unique_cards = set.iter().filter(|v| v != &&0).count();
    if unique_cards == 2 && hand_type == Type::ThreeOfAKind {
        return Type::FullHouse;
    }

    let pairs = set.iter().filter(|v| v == &&2). count();
    match pairs {
        1 => { hand_type = Type::OnePair }
        2 => { hand_type = Type::TwoPairs }
        _ => { }
    }

    return hand_type;
}

fn get_joker_type(hand: &HandJoker) -> Type {
    let jokers = hand.cards.iter().filter(|v| v == &&1).count();
    let normal_type = get_type_2(hand);
    let mut new_type = Type::HighCard;
    match normal_type {
        Type::FiveOfAKind => { },
        Type::FourOfAKind => {
            if jokers == 4 { new_type = Type::FiveOfAKind }
            if jokers == 1 { new_type = Type::FiveOfAKind }
        },
        Type::FullHouse => {
            if jokers == 3 || jokers == 2 {
                new_type = Type::FiveOfAKind
            }
        },
        Type::ThreeOfAKind => {
            if jokers == 3 { new_type = Type::FourOfAKind }
            if jokers == 1 { new_type = Type::FourOfAKind }
        },
        Type::TwoPairs => {
            if jokers == 1 { new_type = Type::FullHouse }
            if jokers == 2 { new_type = Type::FourOfAKind }
        },
        Type::OnePair => {
            if jokers == 1 { new_type = Type::ThreeOfAKind }
            if jokers == 2 { new_type = Type::ThreeOfAKind }
        },
        Type::HighCard => { if jokers == 1 { new_type = Type::OnePair } }
    }
    if new_type > normal_type { new_type } else { normal_type }
}

fn get_value_type(hand: &Hand) -> i64 {
    hand.cards[0] * 100_00_00_00 + hand.cards[1] * 100_00_00 + hand.cards[2] * 100_00 + hand.cards[3] * 100 + hand.cards[4]
}

fn get_value_type_2(hand: &HandJoker) -> i64 {
    hand.cards[0] * 100_00_00_00 + hand.cards[1] * 100_00_00 + hand.cards[2] * 100_00 + hand.cards[3] * 100 + hand.cards[4]
}

fn c_to_n(card: &str) -> [i64; 5] {
    let mut cards: [i64; 5] = [0; 5];
    for (i, c) in card.chars().enumerate() {
        match c {
            'T' => { cards[i] = 10 }
            'J' => { cards[i] = 11 }
            'Q' => { cards[i] = 12 }
            'K' => { cards[i] = 13 }
            'A' => { cards[i] = 14 }
            _ => { cards[i] = c.to_digit(10).unwrap() as i64 }
        }
    }
    cards
}

fn c_to_n_with_j(card: &str) -> [i64; 5] {
    let mut cards: [i64; 5] = [0; 5];
    for (i, c) in card.chars().enumerate() {
        match c {
            'T' => { cards[i] = 10 }
            'J' => { cards[i] = 1 }
            'Q' => { cards[i] = 12 }
            'K' => { cards[i] = 13 }
            'A' => { cards[i] = 14 }
            _ => { cards[i] = c.to_digit(10).unwrap() as i64 }
        }
    }
    cards
}

fn total_winnings(hands: Vec<Hand>) -> i64 {
    let mut winnings = 0;
    let mut sorted_hands = hands.clone();
    sorted_hands.sort();
    for (rank, hand) in sorted_hands.iter().enumerate() {
        winnings += (rank + 1) as i64 * hand.bid;
    }
    winnings

}

fn total_winnings_with_jokers(hands: Vec<HandJoker>) -> i64 {
    let mut winnings = 0;
    let mut sorted_hands = hands.clone();
    println!("{:?}", sorted_hands);
    sorted_hands.sort();
    for (rank, hand) in sorted_hands.iter().enumerate() {
        println!("{}. {:?}, {:?}", rank, hand, get_joker_type(hand));
        winnings += (rank + 1) as i64 * hand.bid;
    }
    winnings

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let hand1 = Hand { cards: c_to_n("32T3K"), bid: 1 };
        let hand2 = Hand { cards: c_to_n("T55J5"), bid: 1 };
        let hand3 = Hand { cards: c_to_n("KK677"), bid: 1 };
        let hand4 = Hand { cards: c_to_n("KTJJT"), bid: 1 };
        let hand5 = Hand { cards: c_to_n("QQQJA"), bid: 1 };

        assert_eq!(hand5 > hand1, true);
        assert_eq!(hand4 > hand1, true);
        assert_eq!(hand3 > hand1, true);
        assert_eq!(hand2 > hand1, true);

        assert_eq!(hand3 > hand4, true);
        assert_eq!(hand5 > hand2, true);
    }

    #[test]
    fn test_get_type() {
        assert_eq!(get_type(&Hand { cards: c_to_n("32T3K"), bid: 765 }), Type::OnePair);
        assert_eq!(get_type(&Hand { cards: c_to_n("T55J5"), bid: 684 }), Type::ThreeOfAKind);
        assert_eq!(get_type(&Hand { cards: c_to_n("KK677"), bid: 28 }), Type::TwoPairs);
        assert_eq!(get_type(&Hand { cards: c_to_n("KTJJT"), bid: 220 }), Type::TwoPairs);
        assert_eq!(get_type(&Hand { cards: c_to_n("QQQJA"), bid: 483 }), Type::ThreeOfAKind);
    }

    #[test]
    fn test_total_winnings() {
        let hands = [
            Hand { cards: c_to_n("32T3K"), bid: 765 },
            Hand { cards: c_to_n("T55J5"), bid: 684 },
            Hand { cards: c_to_n("KK677"), bid: 28 },
            Hand { cards: c_to_n("KTJJT"), bid: 220 },
            Hand { cards: c_to_n("QQQJA"), bid: 483 },
        ];

        assert_eq!(total_winnings(hands.to_vec()), 6440);

        let hands = [
            Hand { cards: c_to_n("2345A"), bid: 1 },
            Hand { cards: c_to_n("Q2KJJ"), bid: 13 },
            Hand { cards: c_to_n("Q2Q2Q"), bid: 19 },
            Hand { cards: c_to_n("T3T3J"), bid: 17 },
            Hand { cards: c_to_n("T3Q33"), bid: 11 },
            Hand { cards: c_to_n("2345J"), bid: 3 },
            Hand { cards: c_to_n("J345A"), bid: 2 },
            Hand { cards: c_to_n("32T3K"), bid: 5 },
            Hand { cards: c_to_n("T55J5"), bid: 29 },
            Hand { cards: c_to_n("KK677"), bid: 7 },
            Hand { cards: c_to_n("KTJJT"), bid: 34 },
            Hand { cards: c_to_n("QQQJA"), bid: 31 },
            Hand { cards: c_to_n("JJJJJ"), bid: 37 },
            Hand { cards: c_to_n("JAAAA"), bid: 43 },
            Hand { cards: c_to_n("AAAAJ"), bid: 59 },
            Hand { cards: c_to_n("AAAAA"), bid: 61 },
            Hand { cards: c_to_n("2AAAA"), bid: 23 },
            Hand { cards: c_to_n("2JJJJ"), bid: 53 },
            Hand { cards: c_to_n("JJJJ2"), bid: 41 },
        ];

        assert_eq!(total_winnings(hands.to_vec()), 6592);

        let hands = [
            HandJoker { cards: c_to_n_with_j("32T3K"), bid: 765 },
            HandJoker { cards: c_to_n_with_j("T55J5"), bid: 684 },
            HandJoker { cards: c_to_n_with_j("KK677"), bid: 28 },
            HandJoker { cards: c_to_n_with_j("KTJJT"), bid: 220 },
            HandJoker { cards: c_to_n_with_j("QQQJA"), bid: 483 },
        ];

        assert_eq!(total_winnings_with_jokers(hands.to_vec()), 5905);

        let hands = [
            HandJoker { cards: c_to_n_with_j("2345A"), bid: 1 },
            HandJoker { cards: c_to_n_with_j("Q2KJJ"), bid: 13 },
            HandJoker { cards: c_to_n_with_j("Q2Q2Q"), bid: 19 },
            HandJoker { cards: c_to_n_with_j("T3T3J"), bid: 17 },
            HandJoker { cards: c_to_n_with_j("T3Q33"), bid: 11 },
            HandJoker { cards: c_to_n_with_j("2345J"), bid: 3 },
            HandJoker { cards: c_to_n_with_j("J345A"), bid: 2 },
            HandJoker { cards: c_to_n_with_j("32T3K"), bid: 5 },
            HandJoker { cards: c_to_n_with_j("T55J5"), bid: 29 },
            HandJoker { cards: c_to_n_with_j("KK677"), bid: 7 },
            HandJoker { cards: c_to_n_with_j("KTJJT"), bid: 34 },
            HandJoker { cards: c_to_n_with_j("QQQJA"), bid: 31 },
            HandJoker { cards: c_to_n_with_j("JJJJJ"), bid: 37 },
            HandJoker { cards: c_to_n_with_j("JAAAA"), bid: 43 },
            HandJoker { cards: c_to_n_with_j("AAAAJ"), bid: 59 },
            HandJoker { cards: c_to_n_with_j("AAAAA"), bid: 61 },
            HandJoker { cards: c_to_n_with_j("2AAAA"), bid: 23 },
            HandJoker { cards: c_to_n_with_j("2JJJJ"), bid: 53 },
            HandJoker { cards: c_to_n_with_j("JJJJ2"), bid: 41 },
        ];

        assert_eq!(total_winnings_with_jokers(hands.to_vec()), 6839);
    }
}