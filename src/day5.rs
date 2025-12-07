use crate::file_utils::fetch_input;
use std::cmp::{max, min};
use std::str::Split;

const DAY: i32 = 5;
const TEST_ANSWER: usize = 14;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}
fn solve_day(contents: String) -> usize {
    let (_ids_to_test, valid_id_ranges) = parse_input(contents);
    let mut reduced_bounds: Vec<(usize, usize)> = Vec::new();
    let mut change_found = true;
    let mut current_bounds = valid_id_ranges.clone();
    while change_found {
        change_found = false;
        reduced_bounds.clear();
        for (lower_bound, upper_bound) in &current_bounds {
            // println!("{} -> {}", lower_bound, upper_bound);
            let mut index_to_remove: Option<usize> = Option::None;
            let mut new_addition: (usize, usize) = (*lower_bound, *upper_bound);
            for (index, (other_lower, other_upper)) in reduced_bounds.iter().enumerate() {
                if other_bound_contained_in_range(lower_bound, upper_bound, other_lower, other_upper) ||
                    other_bound_contained_in_range(other_lower, other_upper, lower_bound, upper_bound)
                {
                    let lowest_bound = min(*lower_bound, *other_lower);
                    let upperest_bound = max(*upper_bound, *other_upper);
                    index_to_remove = Some(index);
                    new_addition = (lowest_bound, upperest_bound);
                    change_found = true;
                    println!(
                        "{}-{} & {}-{} -> {}-{}",
                        lower_bound,
                        upper_bound,
                        other_lower,
                        other_upper,
                        lowest_bound,
                        upperest_bound
                    );
                    break;
                }
            }
            if index_to_remove.is_some() {
                reduced_bounds.remove(index_to_remove.unwrap());
            }
            reduced_bounds.push(new_addition);
        }
        current_bounds = reduced_bounds.clone();
        println!("Looping");
    }
    println!("reduced bounds: {:?}", reduced_bounds);
    let mut count: usize = 0;
    for (lower_bound, upper_bound) in reduced_bounds {
        count += (upper_bound + 1) - lower_bound
    }
    count
}

fn other_bound_contained_in_range(lower: &usize, upper: &usize, other_lower: &usize, other_upper: &usize) -> bool {
    (other_lower >= lower && other_lower <= upper) || (other_upper >= lower && other_upper <= upper)
}

fn parse_input(contents: String) -> (Vec<usize>, Vec<(usize, usize)>) {
    let lines: Vec<&str> = contents.split("\r\n").collect();
    let first_empty_line = lines.iter().position(|x| *x == "").unwrap();
    let split_lines = lines.split_at(first_empty_line);
    let valid_id_lines: Vec<&str> = split_lines.0.iter().map(|x| *x).collect();
    let ids_to_test: Vec<usize> = split_lines
        .1
        .iter()
        .map(|x| *x)
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect();
    let valid_id_ranges: Vec<(usize, usize)> = valid_id_lines
        .iter()
        .map(|x| x.split("-"))
        .map(|mut x| (get_next_parsed_int(&mut x), get_next_parsed_int(&mut x)))
        .collect();
    (ids_to_test, valid_id_ranges)
}

fn get_next_parsed_int(x: &mut Split<&str>) -> usize {
    x.next().unwrap().parse().unwrap()
}
