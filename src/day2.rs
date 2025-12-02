use crate::file_utils::fetch_input;
use std::collections::HashSet;
use std::iter;

const DAY: i32 = 2;
const TEST_ANSWER: usize = 4174379265;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}

fn solve_day(contents: String) -> usize {
    let pairs = get_pairs(&contents);
    let mut invalid_ids: Vec<usize> = Vec::new();
    for pair in pairs {
        let lower_bound: usize = pair[0].parse().unwrap();
        let upper_bound: usize = pair[1].parse().unwrap();
        let lower_bound_length = lower_bound.to_string().len();
        let upper_bound_length = upper_bound.to_string().len();
        // check_lengths_match(lower_bound_length, upper_bound_length);
        let max_number_of_digits_in_prefix = lower_bound_length.div_ceil(2);
        for prefix_divisor in max_number_of_digits_in_prefix..upper_bound_length {
            let prefix_start: usize = get_prefix(lower_bound, prefix_divisor);
            let prefix_end: usize = get_prefix(upper_bound, prefix_divisor);
            for prefix in prefix_start..prefix_end + 1 {
                let prefix_length = prefix.to_string().len();
                let upper_bound_repetitions = upper_bound_length / prefix_length;
                for repetitions in 2..=upper_bound_repetitions {
                    let proposed_id: usize = iter::repeat(prefix.to_string())
                        .take(repetitions)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    // println!("{}", proposed_id);
                    if proposed_id >= lower_bound && proposed_id <= upper_bound {
                        invalid_ids.push(proposed_id)
                    }
                }
            }
        }
    }
    let invalid_ids: Vec<usize> = invalid_ids
        .into_iter()
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect();
    let res = invalid_ids.iter().fold(0, |acc, x| acc + x);
    println!("invalid ids {:?}", invalid_ids);
    println!("sum {}", res);
    res
}

const BASE_EXPONENT: usize = 10;

fn get_prefix(number_to_prefix: usize, divisor_exponent: usize) -> usize {
    let divisor: usize = BASE_EXPONENT.pow((divisor_exponent) as u32);
    number_to_prefix / (divisor)
}

fn get_pairs(contents: &String) -> Vec<Vec<&str>> {
    let unsplit_pairs = contents.split(',').collect::<Vec<&str>>();
    let pairs = unsplit_pairs
        .iter()
        .map(|unsplit_pair| {
            let split = unsplit_pair.split('-').collect::<Vec<&str>>();
            // println!("{} - {}", split[0], split[1]);
            split
        })
        .collect::<Vec<Vec<&str>>>();
    pairs
}