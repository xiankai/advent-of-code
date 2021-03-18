use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::marker::Copy;
use std::collections::HashMap;
use std::cmp::max;

type BagColor = String;

#[derive(Clone)]
struct BagRule {
    color: BagColor,
    amount: usize,
}


struct Bag {
    contains: Vec<BagRule>,
    is_contained_by: Vec<BagRule>,
}

type Bags = HashMap<BagColor, Bag>;

fn read_input(path: &str) -> Result<Bags, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut bags: Bags = HashMap::new();
    for line in br.lines() {
        let y = line?;
        let mut x = y.split("contain");


        let bag_line: Vec<&str> = x.next().unwrap().split(' ').collect();
        let color = [bag_line[0], bag_line[1]].join(" ");

        let rule_lines: Vec<&str> = x.next().unwrap().split(", ").collect();
        let contains = rule_lines.into_iter().map(|rule_line| {
            let parsed_rule_line: Vec<&str> = rule_line.trim().split(' ').collect();

            let rule_color = [parsed_rule_line[1], parsed_rule_line[2]].join(" ");
            let amount = parsed_rule_line[0].parse::<usize>().or::<usize>(Ok(0)).unwrap();

            // attach rule to the color being contained
            let contained_bag = bags.entry(rule_color.clone()).or_insert(Bag {
                contains: vec![],
                is_contained_by: vec![]
            });
            contained_bag.is_contained_by.push(BagRule {
                color: color.clone(),
                amount,
            });

            BagRule {
                color: rule_color,
                amount,
            }
        }).collect();

        let bag = bags.entry(color).or_insert(Bag {
            contains: vec![],
            is_contained_by: vec![],
        });
        bag.contains = contains;
    }
    Ok(bags)
}

fn get_kinds_of_container_bags(all_bags: &Bags, bag_color: BagColor) -> Vec<BagColor> {
    let bag = all_bags.get(&bag_color).unwrap();
    let mut container_bags: Vec<BagColor> = bag.is_contained_by.clone().into_iter().flat_map(|bag_rule| {
        // println!("{} is contained by {}", bag_color, bag_rule.color);
        get_kinds_of_container_bags(all_bags, bag_rule.color)
    }).collect();
    container_bags.push(bag_color);
    container_bags.sort_unstable();
    container_bags.dedup();
    container_bags
}

fn get_sum_of_contained_bags(all_bags: &Bags, bag_color: BagColor) -> usize {
    let bag = all_bags.get(&bag_color).unwrap();
    bag.contains.clone().into_iter().map(|bag_rule| {
        let total = bag_rule.amount * get_sum_of_contained_bags(all_bags, bag_rule.color.clone());
        println!("{} contains {} {} bags for a total of {}", bag_color, bag_rule.amount, bag_rule.color.clone(), total);
        total
    }).sum::<usize>() + 1
}


fn main() {
    let all_bags = read_input("./input.txt").expect("invalid input");

    println!("kinds of bags: {}", get_kinds_of_container_bags(&all_bags, "shiny gold".to_string()).len() - 1);
    println!("total count of bags: {}", get_sum_of_contained_bags(&all_bags, "shiny gold".to_string()) - 1);
}