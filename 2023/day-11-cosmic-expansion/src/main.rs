use common::{read_input_as_char_vectors,get_buffer_reader, transpose, combinations};

type Universe = Vec<Vec<char>>;

fn main() {
    // let br = get_buffer_reader("sample.txt");
    let br = get_buffer_reader("input.txt");
    let input = read_input_as_char_vectors(br).unwrap();

    let mut pt_1_universe = input.clone();
    let universe = expand_universe(&mut pt_1_universe);
    let galaxies = plot_galaxies(universe);
    let combs = combinations(galaxies.len());
    let sum_len = combs.iter().map(|(one, two)| shortest_path(galaxies[*one], galaxies[*two])).sum::<i64>();
    println!("{}", sum_len);

    let pt_2_universe = input.clone();
    let a_new_hope = blow_up_the_universe(&pt_2_universe);
    let combs2 = combinations(a_new_hope.len());
    let sum_len2 = combs2.iter().map(|(one, two)| shortest_path(a_new_hope[*one], a_new_hope[*two])).sum::<i64>();
    println!("{}", sum_len2);
}

fn expand_universe(universe: &mut Universe) -> Universe {
    let expansion_row = vec!['.'; universe[0].len()];
    for row_i in (0..universe.len()).rev() { // go backwards so any addition won't affect subsequent expansions
        let row = &universe[row_i];
        if row.iter().all(|c| *c == '.') {
            universe.splice(row_i..=row_i, vec!(expansion_row.clone(), expansion_row.clone()));
        }
    }

    *universe = transpose(universe.clone());

    let expansion_col = vec!['.'; universe[0].len()];
    for col_i in (0..universe.len()).rev() {
        let col = &universe[col_i];
        if col.iter().all(|c| *c == '.') {
            universe.splice(col_i..=col_i, vec!(expansion_col.clone(), expansion_col.clone()));
        }
    }

    *universe = transpose(universe.clone());

    universe.to_owned()
}

fn blow_up_the_universe(universe: &Universe) -> Vec<(usize, usize)> {
    let old_galaxies = plot_galaxies(universe.clone());
    let mut expansion_rows = vec![0; universe.len()];

    for row_i in 0..universe.len() {
        let row = &universe[row_i];
        if row.iter().all(|c| *c == '.') {
            expansion_rows[row_i] = 1;
        }
    }

    let transposed = transpose(universe.clone());
    let mut expansion_cols = vec![0; transposed.len()];
    for col_i in 0..transposed.len() {
        let col = &transposed[col_i];
        if col.iter().all(|c| *c == '.') {
            expansion_cols[col_i] = 1;
        }
    }

    let mut new_galaxies = vec!();
    for galaxy in old_galaxies {
        let (row, col) = galaxy;
        // new coord = old coord + sum of previous expansion rows * 1m
        new_galaxies.push((
            row.checked_add(expansion_rows[0..row].iter().sum::<usize>() * (1_000_000-1)).unwrap(),
            col.checked_add(expansion_cols[0..col].iter().sum::<usize>() * (1_000_000-1)).unwrap(),
        ));
    }

    new_galaxies
}

fn plot_galaxies(universe: Universe) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for row_i in 0..universe.len() {
        let row = &universe[row_i];
        for col_i in 0..row.len() {
            let col = &row[col_i];
            if *col == '#' {
                galaxies.push((row_i, col_i));
            }
        }
    }

    galaxies
}

fn shortest_path(one: (usize, usize), two: (usize, usize)) -> i64 {
    let (x1, y1) = one;
    let (x2, y2) = two;

    let x = (x1 as i64 - x2 as i64).abs();
    let y = (y1 as i64 - y2 as i64).abs();

    x + y
}