use common::regex::Regex;
use common::{read_input_as_string, get_buffer_reader};
use common::hashbrown::HashMap;
use common::num_integer::lcm;

fn main() {
    // let br = get_buffer_reader("sample3.txt");
    let br = get_buffer_reader("input.txt");
    let input = read_input_as_string(br).unwrap();

    let (instructions, dict, ghost_nodes) = parse_input(input);
    println!("{:?}", navigate(&instructions, &dict));

    println!("{:?}", ghost_nodes);

    println!("{:?}", navigate_as_a_ghost(&instructions, &dict, ghost_nodes));
}

type Map = HashMap<String, [String; 2]>;
type GhostNodes = Vec<String>;


fn parse_input(input: Vec<String>) -> (String, Map, GhostNodes) {
    let re = Regex::new(r"(.*)").unwrap();
    let instruction = re.find(&input[0]).unwrap().as_str();
    let mut ghost_nodes = vec!();

    let re2 = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut dict = HashMap::new();
    input[1..].iter().for_each(|line| {
        re2.captures_iter(line).for_each(|cap| {
            dict.insert(cap[1].to_owned(), [cap[2].to_owned(), cap[3].to_owned()]);

            // for part 2 only
            if cap[1].chars().nth(2) == Some('A') {
                ghost_nodes.push(cap[1].to_owned());
            }
        });
    });
    (instruction.to_owned(), dict, ghost_nodes)
}

fn navigate(instructions: &String, dict: &Map) -> i64 {
    let mut current_pos = "AAA";
    let mut steps: i64 = 0;

    while current_pos != "ZZZ" {
        instructions.chars().for_each(|char| {
            if current_pos == "ZZZ" {
                return;
            }
            steps += 1;
            current_pos = match char {
                'L' => &dict.get(current_pos).unwrap()[0],
                'R' => &dict.get(current_pos).unwrap()[1],
                _ => panic!("??"),
            };
        });
    }

    steps
}

fn navigate_as_a_ghost(instructions: &String, dict: &Map, ghost_nodes: Vec<String>) -> i64 {
    let mut iterators = ghost_nodes;
    let mut steps: i64 = 0;

    let mut first_zs: Vec<Vec<i64>> = vec![vec!(); iterators.len()];

    while !first_zs.iter().all(|vec| vec.len() >= 1) {
        instructions.chars().for_each(|char| {
            iterators.iter().enumerate().for_each(|(i, test_str)| {
                if test_str.chars().nth(2) == Some('Z') {
                    first_zs[i].push(steps);
                }
            });
            steps += 1;
            iterators = iterators.iter().map(|node| {
                let new_node = match char {
                    'L' => &dict.get(node).unwrap()[0],
                    'R' => &dict.get(node).unwrap()[1],
                    _ => panic!("??"),
                };
                new_node.to_owned()
            }).collect();
        });
    }

    // println!("{:?}", first_zs);
    first_zs.iter().fold(1, |acc, vec| {
        lcm(vec[0], acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
    }
}
