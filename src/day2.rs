use crate::file_utils::fetch_input;

const DAY: i32 = 2;
const TEST_ANSWER: i32 = 6;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}

fn solve_day(contents: String) -> i32 {
    println!("{}", contents);
    2
}


