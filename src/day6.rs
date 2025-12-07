use std::str::Chars;
use crate::file_utils::fetch_input;

const DAY: i32 = 6;
const TEST_ANSWER: usize = 3263827;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}
fn solve_day(contents: String) -> usize {
    let input = parse_input(contents);
    println!("{:?}", input);
    let mut total = 0;
    for (numbers, operator) in input {
        let mut column_total: usize = 0;
        if operator == '+' {
            column_total = numbers.iter().fold(0, |acc, x| acc + x);
        } else {
            column_total = numbers.iter().fold(1, |acc, x| acc * x);
        }
        total += column_total;
    }
    total
}

fn parse_input(contents: String) -> Vec<(Vec<usize>, char)> {
    let lines: Vec<&str> = contents
        .split("\r\n")
        .filter(|x| *x != "")
        .collect::<Vec<&str>>();
    if let Some((operator_line, number_lines)) = lines.split_last() {
        let mut chars = operator_line.chars().into_iter();
        let mut number_line_chars: Vec<Chars> = number_lines.iter().map(|x| x.chars().into_iter()).collect();
        let mut input: Vec<(Vec<usize>, char)> = Vec::new();
        let mut numbers: Vec<usize> = Vec::new();
        let mut operator: Option<char> = None;
        loop {
            let operator_or_space = chars.next();
            if operator_or_space.is_some() && (operator_or_space.unwrap() != ' ') {
                operator = Some(operator_or_space.unwrap());
            }
            let mut num = String::new();
            for line in &mut number_line_chars {
                let take = &line.next();
                if take.is_some() && take.unwrap() != ' ' {
                    num = format!("{}{}", num, take.unwrap());
                }
            }
            if (&num).is_empty() {
                input.push((numbers, operator.unwrap()));
                numbers = Vec::new();
                operator = None;
                if (operator_or_space.is_none()) {
                    break;
                }
            } else {
                numbers.push(num.parse::<usize>().unwrap());
            }
        }
        input
    } else {
        panic!("Not enough lines found")
    }
}
