use crate::day7::Square::{Beam, EmptySpace, Manifold, Splitter};
use crate::file_utils::fetch_input;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use std::mem::replace;
use std::ops::Index;
use std::str::Chars;

const DAY: i32 = 7;
const TEST_ANSWER: usize = 21;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}

fn solve_day(contents: String) -> usize {
    let mut input: Vec<Vec<Square>> = parse_input(contents);
    let manifold_index = input
        .get(0)
        .unwrap()
        .iter()
        .position(|x| *x == Manifold)
        .unwrap();
    let mut x: HashMap<usize, usize> = HashMap::new();
    let number_of_columns = input.get(0).unwrap().len();
    let mut count = 0;
    print_state(&input);
    for y in 1..input.len() {
        let line = input.get(y).unwrap();
        for x in 0..line.len() {
            update_square(&mut input, y, x, &mut count);
        }
        print_state(&input);
    }
    println!("{}", count);

    count
}

fn update_square(mut input: &mut Vec<Vec<Square>>, y: usize, x: usize, count: &mut usize) {
    let sq = get_square(&input, y, x);
    let above = get_square(&input, y - 1, x);
    if sq == EmptySpace {
        if above == Manifold || above == Beam {
            set_square(&mut input, y, x, Beam);
        }
    } else if sq == Splitter {
        if above == Manifold || above == Beam {
            set_square(&mut input, y, x-1, Beam);
            set_square(&mut input, y, x+1, Beam);
            *count += 1
        }
    }
}

fn set_square(input: &mut Vec<Vec<Square>>, y: usize, x: usize, new_value: Square) {
    let _ = replace(input.get_mut(y).unwrap().get_mut(x).unwrap(), new_value);
}

fn get_square(input: &Vec<Vec<Square>>, y: usize, x: usize) -> &Square {
    input.get(y).unwrap().get(x).unwrap()
}

fn print_state(input: &Vec<Vec<Square>>) {
    let columns = input.get(0).unwrap().len();
    for y in 0..input.len() {
        for x in 0..columns {
            print!("{}", input.get(y).unwrap().get(x).unwrap())
        }
        print!("\n")
    }
    print!("\n");
    for _i in 0..columns {
        print!("-");
    }
    print!("\n\n")
}

fn parse_input(contents: String) -> Vec<Vec<Square>> {
    let lines: Vec<Vec<Square>> = contents
        .split("\r\n")
        .filter(|x| *x != "")
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => EmptySpace,
                    'S' => Manifold,
                    '^' => Splitter,
                    _ => panic!("Unknown type {}", char),
                })
                .collect::<Vec<Square>>()
        })
        .collect();
    lines
}

#[derive(PartialEq, Eq)]
enum Square {
    Manifold,
    Splitter,
    EmptySpace,
    Beam,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Manifold => write!(f, "S"),
            Splitter => write!(f, "^"),
            EmptySpace => write!(f, "."),
            Beam => write!(f, "|"),
        }
    }
}
impl PartialEq<Square> for &Square {
    fn eq(&self, other: &Square) -> bool {
        **self == *other
    }
}
