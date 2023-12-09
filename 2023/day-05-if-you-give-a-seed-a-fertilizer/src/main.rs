use common::read_input_as_string;
use thousands::Separable;

use std::fs::File;
use std::io::BufReader;

use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);

    let input = read_input_as_string(br).unwrap();
    let almanac = parse_almanac(&input);
    // let lowest_location = get_lowest_location(almanac);
    // println!("Lowest location: {}", lowest_location);

    let seed_ranges = parse_seed_ranges_by_ranges(input[0].clone());
    let seed_range = combine_seed_ranges_into_one_giant_breakpoint(seed_ranges);
    let breakpoints = get_breakpoints_for_seed_range(seed_range, &almanac.1);
    println!("Breakpoints: {:?}", breakpoints);
    let lowest_location = get_lowest_location_by_breakpoints(breakpoints, almanac);
    println!("Lowest location: {}", lowest_location);
}

#[derive(Clone, Debug)]
struct Range {
    from: i64,
    to: i64,
    range: i64,
}

#[derive(Clone, Debug)]
struct Map {
    ranges: Vec<Range>,
}

fn _parse_seed_ranges(input: String) -> Vec<i64> {
    let seed_regex = Regex::new(r"(\d+) (\d+)").unwrap();
    let seeds = seed_regex
        .captures_iter(&input)
        .map(|captures| {
            let from = captures[1].parse::<i64>().unwrap();
            let to = captures[2].parse::<i64>().unwrap();
            from..from + to
        })
        .flatten()
        .collect();
    seeds
}

fn parse_seed_ranges_by_ranges(input: String) -> Vec<(i64, i64)> {
    let seed_regex = Regex::new(r"(\d+) (\d+)").unwrap();
    let seeds = seed_regex
        .captures_iter(&input)
        .map(|captures| {
            let from = captures[1].parse::<i64>().unwrap();
            let to = captures[2].parse::<i64>().unwrap();
            (from, from+to-1)
        })
        .collect();
    seeds
}

fn combine_seed_ranges_into_one_giant_breakpoint(seed_ranges: Vec<(i64, i64)>) -> Vec<i64> {
    let mut breakpoints: Vec<i64> = vec!();
    for seed_range in seed_ranges {
        breakpoints.push(seed_range.0);
        breakpoints.push(seed_range.1);
    }
    breakpoints.sort();
    breakpoints
}

fn parse_almanac(input: &Vec<String>) -> (Vec<i64>, Vec<Map>) {
    let mut iterator = input.iter();
    let seed_regex = Regex::new(r"(\d+)").unwrap();
    let raw_seeds = iterator.next();
    let seeds = seed_regex
        .find_iter(raw_seeds.unwrap())
        .map(|seed| seed.as_str().parse::<i64>().unwrap())
        .collect();

    let mut maps = Vec::new();
    let map_regex = Regex::new(r"(.*)-to-(.*) map").unwrap();
    let range_regex = Regex::new(r"(.+) (.+) (.+)").unwrap();
    let mut current_map: Option<Map> = None;
    iterator.for_each(|line| {
        if map_regex.is_match(line) {
            if let Some(old_map) = &current_map {
                maps.push(old_map.clone());
            }
            current_map = Some(Map {
                ranges: Vec::new(),
            });
        }

        if range_regex.is_match(line) {
            let mut iter = range_regex.find(line).unwrap().as_str().split(" ");
            // wth, destination range is first, source range is second
            let to = iter.next().unwrap().parse::<i64>().unwrap();
            let from = iter.next().unwrap().parse::<i64>().unwrap();
            let range = iter.next().unwrap().parse::<i64>().unwrap();
            current_map.as_mut().unwrap().ranges.push(Range {
                from: from,
                to: to,
                range: range,
            });
        }
    });
    (seeds, maps)
}

fn lookup_map(from: i64, map: &Map) -> i64 {
    for range in map.ranges.iter() {
        let checking = range.from..=(range.from + range.range);
        if checking.contains(&from) {
            let offset = from - range.from;
            return range.to + offset;
        }
    }
    return from;
}

fn lookup_map_by_breakpoints(breakpoints: Vec<i64>, map: &Map) ->  Vec<i64> {
    let mut temp1 = breakpoints.clone();
    let temp2: &mut Vec<i64> = &mut temp1;
    let final_breakpoints = map.ranges.iter().fold(temp2, |wip, range| {
        get_range_breakpoints(wip, (range.from, range.from+range.range))
    });
    let mut computed_breakpoints = final_breakpoints.clone().iter().map(|breakpoint| lookup_map(*breakpoint, map)).collect::<Vec<i64>>();
    computed_breakpoints.sort();
    computed_breakpoints

}

fn get_range_breakpoints(breakpoints: &mut Vec<i64>, range2: (i64, i64)) -> &mut Vec<i64> {
    let mut to_push = vec!();
    let min = breakpoints[0];
    let max = breakpoints[breakpoints.len() - 1];
    if range2.0 > min && range2.0 < max && !breakpoints.contains(&range2.0) {
        to_push.push(range2.0);
    }
    if range2.1 > min && range2.1 < max && !breakpoints.contains(&range2.1) {
        to_push.push(range2.1);
    }
    if to_push.len() == 0 {
        return breakpoints;
    }
    breakpoints.append(&mut to_push);
    breakpoints.sort();
    breakpoints
}

fn get_location_for_seed(seed: i64, maps: &Vec<Map>) -> i64 {
    let mut from = seed;
    for map in maps {
        from = lookup_map(from, map);
    }
    from
}

fn _get_lowest_location(almanac: (Vec<i64>, Vec<Map>)) -> i64 {
    let mut lowest = i64::MAX;
    for seed in almanac.0 {
        let location = get_location_for_seed(seed, &almanac.1);
        if location < lowest {
            lowest = location;
        }
    }
    lowest
}

fn get_breakpoints_for_seed_range(breakpoints: Vec<i64>, maps: &Vec<Map>) -> Vec<i64> {
    let mut breakpoints: Vec<i64> = breakpoints.clone();
    for map in maps {
        breakpoints = lookup_map_by_breakpoints(breakpoints, map);
    }
    breakpoints
}

fn get_lowest_location_by_breakpoints(breakpoints: Vec<i64>, almanac: (Vec<i64>, Vec<Map>)) -> i64 {
    let mut lowest = i64::MAX;
    for seed in breakpoints {
        // let location = get_location_for_seed(seed, &almanac.1);
        // let location = lookup_map(seed, &almanac.1.last().unwrap());
        let location = seed;
        if location < lowest {
            lowest = location;
        }
    }
    lowest
}

fn expand_breakpoints(breakpoints: Vec<i64>) -> Vec<i64> {
    let mut expanded: Vec<i64> = vec!();
    for breakpoint in breakpoints {
        expanded.push(breakpoint);
        expanded.push(breakpoint -1);
        expanded.push(breakpoint +1);
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_almanac() {
        let file = File::open("sample.txt").unwrap();
        let br = BufReader::new(file);

        let input = read_input_as_string(br).unwrap();
        let almanac = parse_almanac(&input);
        println!("{:?}", almanac);
    }

    #[test]
    fn test_lookup_map() {
        assert_eq!(
            lookup_map(
                79,
                &Map {
                    ranges: vec![
                        Range {
                            to: 50,
                            from: 98,
                            range: 2,
                        },
                        Range {
                            to: 52,
                            from: 50,
                            range: 48,
                        },
                    ],
                }
            ),
            81
        );
        assert_eq!(
            lookup_map(
                14,
                &Map {
                    ranges: vec![
                        Range {
                            to: 50,
                            from: 98,
                            range: 2,
                        },
                        Range {
                            to: 52,
                            from: 50,
                            range: 48,
                        },
                    ],
                }
            ),
            14
        );
        assert_eq!(
            lookup_map(
                55,
                &Map {
                    ranges: vec![
                        Range {
                            to: 50,
                            from: 98,
                            range: 2,
                        },
                        Range {
                            to: 52,
                            from: 50,
                            range: 48,
                        },
                    ],
                }
            ),
            57
        );
        assert_eq!(
            lookup_map(
                13,
                &Map {
                    ranges: vec![
                        Range {
                            to: 50,
                            from: 98,
                            range: 2,
                        },
                        Range {
                            to: 52,
                            from: 50,
                            range: 48,
                        },
                    ],
                }
            ),
            13
        );
    }

    #[test]
    fn test_get_location_for_seed() {
        let file = File::open("sample.txt").unwrap();
        let br = BufReader::new(file);

        let input = read_input_as_string(br).unwrap();
        let almanac = parse_almanac(&input);
        assert_eq!(get_location_for_seed(79, &almanac.1), 82);
        assert_eq!(get_location_for_seed(14, &almanac.1), 43);
        assert_eq!(get_location_for_seed(55, &almanac.1), 86);
        assert_eq!(get_location_for_seed(13, &almanac.1), 35);
    }

    #[test]
    fn test_get_lowest_location() {
        let file = File::open("sample.txt").unwrap();
        let br = BufReader::new(file);

        let input = read_input_as_string(br).unwrap();
        let almanac = parse_almanac(&input);
        assert_eq!(_get_lowest_location(almanac), 35);
    }

    #[test]
    fn test_parse_seed_ranges() {
        let input = read_input_as_string(Cursor::new("79 14 55 13")).unwrap();
        assert_eq!(
            _parse_seed_ranges(input[0].clone()),
            [
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
                62, 63, 64, 65, 66, 67
            ]
        );
    }

    #[test]
    fn test_get_range_breakpoints() {
        // left overlap
        assert_eq!(get_range_breakpoints(&mut vec![1, 5], (2, 6)), &mut vec![1, 2, 5]);
        // right overlap
        assert_eq!(get_range_breakpoints(&mut vec![3, 7], (2, 6)), &mut vec![3, 6, 7]);
        // inner overlap
        assert_eq!(get_range_breakpoints(&mut vec![1, 5], (2, 4)), &mut vec![1, 2, 4, 5]);
        // outer overlap
        assert_eq!(get_range_breakpoints(&mut vec![1, 5], (0, 6)), &mut vec![1, 5]);
        // no match
        assert_eq!(get_range_breakpoints(&mut vec![1, 5], (6, 10)), &mut vec![1, 5]);

        // left
        assert_eq!(get_range_breakpoints(&mut vec![1, 5, 7], (2, 6)), &mut vec![1, 2, 5, 6, 7]);
        assert_eq!(get_range_breakpoints(&mut vec![1, 5, 6], (2, 6)), &mut vec![1, 2, 5, 6]);

        // right
        assert_eq!(get_range_breakpoints(&mut vec![1, 3, 7], (2, 6)), &mut vec![1, 2, 3, 6, 7]);
        assert_eq!(get_range_breakpoints(&mut vec![2, 3, 7], (2, 6)), &mut vec![2, 3, 6, 7]);
    }

    #[test]
    fn test_get_lowest_location_by_breakpoint() {
        let file = File::open("sample.txt").unwrap();
        let br = BufReader::new(file);

        let input = read_input_as_string(br).unwrap();
        let almanac = parse_almanac(&input);

        let seed_ranges = parse_seed_ranges_by_ranges(input[0].clone());
        println!("{:?}", seed_ranges);
        let seed_range = combine_seed_ranges_into_one_giant_breakpoint(seed_ranges);
        let breakpoints = get_breakpoints_for_seed_range(seed_range, &almanac.1);
        let lowest_location = get_lowest_location_by_breakpoints(breakpoints, almanac);
        assert_eq!(lowest_location, 46);
    }
}
