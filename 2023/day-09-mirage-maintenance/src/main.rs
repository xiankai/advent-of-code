use common::{read_input_as_number_vectors, get_buffer_reader};

fn main() {
    // let br = get_buffer_reader("sample.txt");
    let br = get_buffer_reader("input.txt");
    let inputs = read_input_as_number_vectors(br).unwrap();

    println!("{:?}", inputs.iter().map(|i| history(i)).sum::<i64>());
    println!("{:?}", inputs.iter().map(|i| even_more_history(i)).sum::<i64>());
}

fn generate_diff(input: &Vec<i64>) -> Vec<i64> {
    let mut diff = vec!();
    for i in 0..input.len() - 1 {
        diff.push(input[i + 1] - input[i]);
    }
    diff
}

fn history(input: &Vec<i64>) -> i64 {
    let mut temp = input.clone();
    let mut sum = temp[temp.len() - 1];
    while !temp.iter().all(|x| x == &0) {
        temp = generate_diff(&temp);
        sum += temp[temp.len() - 1];
    }
    sum
}

fn even_more_history(input: &Vec<i64>) -> i64 {
    let mut temp = input.clone();
    let mut stack = vec!(temp[0]);
    while !temp.iter().all(|x| x == &0) {
        temp = generate_diff(&temp);
        stack.push(temp[0]);
    }

    let mut recent = 0;
    for i in (0..stack.len() - 1).rev() {
        let item = stack[i];
        recent = item - recent;
    }
    recent
}