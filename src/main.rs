use std::fs;

const DAY: i32 = 1;
const TEST_ANSWER: i32 = 6;

fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}

fn solve_day(contents: String) -> i32 {
    let split = contents.split("\r\n");
    let iterator = split.into_iter();
    let mut dial = 50;
    let mut passes_of_0 = 0;
    iterator.for_each(|line| {
        let starting_dial = dial;
        let instruction = line.split_at(1);
        println!("{} - {}", dial, line);
        let direction = instruction.0;
        let amount = instruction.1.parse::<i32>().unwrap();
        if direction == "L" {
            dial -= amount;
        } else {
            dial += amount;
        }

        if starting_dial == 0 && direction == "L" {
            passes_of_0 -= 1;
        }

        if starting_dial == 100 && direction == "R" {
            passes_of_0 -= 1;
        }
        while dial < 0 {
            dial += 100;
            passes_of_0 += 1;
        }
        while dial > 100 {
            dial -= 100;
            passes_of_0 += 1;
        }
        if dial == 100 || dial == 0 {
            passes_of_0 += 1;
        }

    });
    passes_of_0
}

fn fetch_input(day: i32, test: bool) -> String {
    let test_or_input = if test { "test" } else { "input" };
    let path = format!("input/DAY{}_{}.txt", day, test_or_input);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    contents
}
