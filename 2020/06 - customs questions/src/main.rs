use std::fs::File;
use std::io::{BufReader, BufRead, Error};

// part 1
fn compute_question_count_unique(mut questions: Vec<Vec<char>>) -> Vec<char> {
    let mut grouped_questions: Vec<char> = questions.into_iter().flatten().collect();
    grouped_questions.sort_unstable();
    grouped_questions.dedup();
    grouped_questions
}

// part 2
fn compute_question_count_intersection(questions: Vec<Vec<char>>) -> Vec<char> {
    let mut common_questions: Vec<char> = questions[0].clone();
    for person in questions.into_iter() {
        let mut new_common_set = vec![];
        // look at this person's questions
        for question in person.into_iter() {
            if common_questions.contains(&question) {
                // this person has answered a common question
                new_common_set.push(question);
            }
        }
        // compute new set of common questions based on this person
        common_questions = new_common_set;
    }
    common_questions
}

fn read_input(path: &str) -> Result<Vec<Vec<char>>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut group_questions: Vec<Vec<char>> = vec![];
    let mut questions: Vec<Vec<char>> = vec![];
    for line in br.lines() {
        let y = line?;
        if y.is_empty() {
            group_questions.push(compute_question_count_intersection(questions));
            questions = vec![];
            continue;
        }

        questions.push(y.chars().collect());
    }

    group_questions.push(compute_question_count_intersection(questions));

    Ok(group_questions)
}

fn main() {
    let v = read_input("./input.txt").expect("invalid input");

    let mut total_questions = 0;
    for group in v.iter() {
        total_questions += group.len();
    }

    println!("Total count of questions: {}", total_questions);
}