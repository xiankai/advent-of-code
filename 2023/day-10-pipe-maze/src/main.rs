use common::{read_input_as_char_vectors, get_buffer_reader};
use common::hashbrown::HashMap;

fn main() {
    let br = get_buffer_reader("input.txt");
    // let br = get_buffer_reader("sample_simple.txt");
    let input = read_input_as_char_vectors(br).unwrap();

    let farthest_point = map_out(&input);
    println!("Farthest point: {}", farthest_point);

    let polygon_chain = flush_out(&input);
    // Apply formula to get area of an enclosed sorted polygon
    let enclosed_area = shoelace_formula(&polygon_chain);

    // 'straight pipe' contributes 1/2 of area
    // 'outer bend' contributes 1/4 of area
    // 'inner bend' contributes 3/4 of area
    // there are 4 more outer bends than inner bends
    println!("Enclosed area: {}", enclosed_area - farthest_point + 1);
}

type Maze = Vec<Vec<char>>;
type Mapped = Vec<Vec<Option<i64>>>;
type Coords = (usize, usize, i64);
type Direction = char;
type Dict = HashMap<(char, char, char), bool>;

// DFS
fn map_out(maze: &Maze) -> i64 {
    let start = get_starting_pos(&maze);
    let mut visited: Mapped = vec![vec![None; maze[0].len()]; maze.len()];

    // Check if pipes can connect
    let allowed_connections = [
        // Origin
        ('S',vec!['N','S','E','W']),
        ('|',vec!['N','S']),
        ('-',vec!['E','W']),
        ('L',vec!['N','E']),
        ('J',vec!['N','W']),
        ('7',vec!['S','W']),
        ('F',vec!['S','E']),
    ];

    let mut get_ent_for_exit = HashMap::new();
    get_ent_for_exit.insert('N', 'S');
    get_ent_for_exit.insert('S', 'N');
    get_ent_for_exit.insert('E', 'W');
    get_ent_for_exit.insert('W', 'E');

    // Create the lookup table just once
    let mut dict = HashMap::new();
    for (from, exits) in allowed_connections.iter() {
        for exit in exits.iter() {
            let ent = get_ent_for_exit.get(exit).unwrap();
            for (to, _) in allowed_connections.iter().filter(|(_, ents)| ents.contains(ent)) {
                dict.insert((*from, *exit, *to), true);
            }
        }
    }

    let mut deepest = 0;
    let mut stack = vec![vec![(start.0, start.1, 0)]];
    while stack.len() > 0 {
        let current_batch = stack.pop().unwrap();
        if current_batch.len() == 0 {
            break;
        }
        let depth = current_batch[0].2;
        if depth > deepest {
            deepest = depth;
        }

        let mut next_batch = vec!();
        for current in current_batch.iter() {
            visited[current.0][current.1] = Some(depth);
            if is_valid_path(current, 'N', &maze, &visited, &dict) {
                next_batch.push((current.0 - 1, current.1, depth + 1));
            }
            if is_valid_path(current, 'S', &maze, &visited, &dict) {
                next_batch.push((current.0 + 1, current.1, depth + 1));
            }
            if is_valid_path(current, 'E', &maze, &visited, &dict) {
                next_batch.push((current.0, current.1 + 1, depth + 1));
            }
            if is_valid_path(current, 'W', &maze, &visited, &dict) {
                next_batch.push((current.0, current.1 - 1, depth + 1));
            }
        }
        stack.push(next_batch);
    }

    deepest
}

// BFS
fn flush_out(maze: &Maze) -> Vec<(usize, usize, i64)> {
    let start = get_starting_pos(&maze);
    let mut visited: Mapped = vec![vec![None; maze[0].len()]; maze.len()];

    // Check if pipes can connect
    let allowed_connections = [
        // Origin
        ('S',vec!['N','S','E','W']),
        ('|',vec!['N','S']),
        ('-',vec!['E','W']),
        ('L',vec!['N','E']),
        ('J',vec!['N','W']),
        ('7',vec!['S','W']),
        ('F',vec!['S','E']),
    ];

    let mut get_ent_for_exit = HashMap::new();
    get_ent_for_exit.insert('N', 'S');
    get_ent_for_exit.insert('S', 'N');
    get_ent_for_exit.insert('E', 'W');
    get_ent_for_exit.insert('W', 'E');

    // Create the lookup table just once
    let mut dict = HashMap::new();
    for (from, exits) in allowed_connections.iter() {
        for exit in exits.iter() {
            let ent = get_ent_for_exit.get(exit).unwrap();
            for (to, _) in allowed_connections.iter().filter(|(_, ents)| ents.contains(ent)) {
                dict.insert((*from, *exit, *to), true);
            }
        }
    }

    let mut depth = 0;
    let mut previous_stack_len = 0;
    let mut stack = vec![(start.0, start.1, 0)];
    while stack.len() > previous_stack_len { // while there are new elements in the stack
        previous_stack_len = stack.len(); // this is our new previous stack length
        depth += 1;
        let current = &stack.last().unwrap().to_owned();
        visited[current.0][current.1] = Some(depth);
        if is_valid_path(current, 'N', &maze, &visited, &dict) {
            stack.push((current.0 - 1, current.1, depth + 1));
        }
        else if is_valid_path(current, 'S', &maze, &visited, &dict) {
            stack.push((current.0 + 1, current.1, depth + 1));
        }
        else if is_valid_path(current, 'E', &maze, &visited, &dict) {
            stack.push((current.0, current.1 + 1, depth + 1));
        }
        else if is_valid_path(current, 'W', &maze, &visited, &dict) {
            stack.push((current.0, current.1 - 1, depth + 1));
        }
    }
    stack
}

fn shoelace_formula(polygon: &Vec<(usize, usize, i64)>) -> i64 {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let n = polygon.len();
    for i in 0..n {
        let j = (i + 1) % n;
        sum1 += polygon[i].0 * polygon[j].1;
        sum2 += polygon[j].0 * polygon[i].1;
    }
    sum1.abs_diff(sum2) as i64 / 2
}

fn is_valid_path(from: &Coords, direction: Direction, maze: &Maze, visited: &Mapped, dict: &Dict) -> bool {
    let to_i64 = match direction {
        'N' => (from.0 as i64 - 1, from.1 as i64),
        'S' => (from.0 as i64 + 1, from.1 as i64),
        'E' => (from.0 as i64, from.1 as i64 + 1),
        'W' => (from.0 as i64, from.1 as i64 - 1),
        _ => panic!("Invalid direction"),
    };

    // First check if destination is out of bounds
    if to_i64.0 >= maze.len() as i64 || to_i64.1 >= maze.len() as i64 || to_i64.0 < 0 || to_i64.1 < 0 {
        return false;
    }

    let to = (to_i64.0 as usize, to_i64.1 as usize);

    // Check if destination has been visited
    if visited[to.0][to.1].is_some() {
        return false;
    }

    let from_char = maze[from.0][from.1];
    let to_char = maze[to.0][to.1];


    // Check if the pipe can connect to the direction we desire, and that the destination accepts it
    let result = dict.contains_key(&(from_char, direction, to_char));
    result
}

fn get_starting_pos(maze: &Maze) -> Coords {
    for (i, row) in maze.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (i, j, 0);
            }
        }
    }
    panic!("No starting point found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let br = get_buffer_reader("sample_simple.txt");
        let input = read_input_as_char_vectors(br).unwrap();
        let result = map_out(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_complex() {
        let br = get_buffer_reader("sample_complex.txt");
        let input = read_input_as_char_vectors(br).unwrap();
        let result = map_out(&input);
        assert_eq!(result, 8);
    }
}