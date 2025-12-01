use std::fs;

fn main() {
    println!("Hello, world!");
    let day = 1;
    let test = true;
    let contents = fetch_input(day, test);
    let day_1_test_answer = solve_day_1(contents);
    assert_eq!(day_1_test_answer, 3);
}

fn solve_day_1(contents: String) -> i32 {
    2
}

fn fetch_input(day: i32, test: bool) -> String {
    let test_or_input = if test { "test" } else { "input" };
    let path = format!("input/day{}_{}.txt", day, test_or_input);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    contents
}
