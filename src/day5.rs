use crate::file_utils::fetch_input;
use std::ops::Index;
use std::str::Split;

const DAY: i32 = 5;
const TEST_ANSWER: usize = 3;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}
fn solve_day(contents: String) -> usize {
    let (ids_to_test, valid_id_ranges) = parse_input(contents);
    println!("{:?}", ids_to_test);
    println!("{:?}", valid_id_ranges);
    let mut count: usize = 0;
    for under_test in ids_to_test {
        let mut is_valid = false;
        for (_, (lower_bound, upper_bound)) in valid_id_ranges.iter().enumerate() {
            if (under_test >= *lower_bound && under_test <= *upper_bound) {
                is_valid = true;
            }
        }
        if is_valid {
            count += 1;
            println!("Valid ID found: {}", under_test);
        }
    }
    count
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
