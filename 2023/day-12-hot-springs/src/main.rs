use common::combinations;

fn main() {
    println!("Hello, world!");
}

type ContiguousDamages = Vec<i64>;
type PossibleCombinations = Vec<Vec<char>>;

fn generate_possible_combinations(damages: ContiguousDamages, length: i64) -> PossibleCombinations {
    let template = vec!();
    for i in damages {
        template.append(&mut vec!['#'; i as usize]);
    }
    // subtract the positions held by damages, and the positions between those damages
    let possible_insertions = length - damages.iter().sum::<i64>() - damages.len() as i64 - 1;

    ()
}