use crate::day7::SquareType::{Beam, EmptySpace, Manifold, Splitter};
use crate::file_utils::fetch_input;
use std::any::Any;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem::replace;
use std::ops::Index;
use std::str::Chars;

const DAY: i32 = 7;
const TEST_ANSWER: usize = 40;

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
    print_state(&input);
    for y in 1..input.len() {
        let line = input.get(y).unwrap();
        for x in 0..line.len() {
            update_square(&mut input, y, x);
        }
    }
    print_state(&input);
    let mut count = 0;
    input[input.len() - 1].iter().for_each(|square| count += square.count);
    println!("{}", count);

    count
}

fn update_square(mut input: &mut Vec<Vec<Square>>, y: usize, x: usize) {
    let sq = get_square(&input, y, x).unwrap();
    let above = get_square(&input, y - 1, x).unwrap();
    let right = get_square(&input, y, x + 1);
    let right_diag = get_square(&input, y - 1, x + 1);
    let mut count: usize = 0;
    if sq.role == EmptySpace {
        if above.role == Manifold || above.role == Beam {
            count += above.count;
        }
        if (x > 0) {
            let left = get_square(&input, y, x - 1);
            let left_diag = get_square(&input, y - 1, x - 1);
            if left.is_some() && left.unwrap().role == Splitter && left_diag.unwrap().role == Beam {
                count += left_diag.unwrap().count;
            }
        }
        if x < input.get(0).unwrap().len() {
            if right.is_some()
                && right.unwrap().role == Splitter
                && right_diag.unwrap().role == Beam
            {
                count += right_diag.unwrap().count
            }
        }
        if count > 0 {
            set_square(&mut input, y, x, Square { role: Beam, count });
        }
    }
}

fn set_square(input: &mut Vec<Vec<Square>>, y: usize, x: usize, new_value: Square) {
    let _ = replace(input.get_mut(y).unwrap().get_mut(x).unwrap(), new_value);
}

fn get_square(input: &Vec<Vec<Square>>, y: usize, x: usize) -> Option<&Square> {
    input.get(y).unwrap().get(x)
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
                    '.' => Square {
                        role: EmptySpace,
                        count: 0,
                    },
                    'S' => Square {
                        role: Manifold,
                        count: 1,
                    },
                    '^' => Square {
                        role: Splitter,
                        count: 0,
                    },
                    _ => panic!("Unknown type {}", char),
                })
                .collect::<Vec<Square>>()
        })
        .collect();
    lines
}

#[derive(PartialEq, Eq)]
struct Square {
    role: SquareType,
    count: usize,
}

#[derive(PartialEq, Eq)]
enum SquareType {
    Manifold,
    Splitter,
    EmptySpace,
    Beam,
}

impl Display for SquareType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Manifold => write!(f, "S"),
            Splitter => write!(f, "^"),
            EmptySpace => write!(f, "."),
            Beam => write!(f, "|"),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.role)
    }
}
impl PartialEq<Square> for &Square {
    fn eq(&self, other: &Square) -> bool {
        **self == *other
    }
}
