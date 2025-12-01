use std::fs;

fn main() {
    println!("Hello, world!");
    let day = 1;
    let test_contents = fetch_input(day, true);
    let day_1_test_answer = solve_day_1(test_contents);
    assert_eq!(day_1_test_answer, 3);
    let real_contents = fetch_input(day, false);
    let day_1_real_answer = solve_day_1(real_contents);
    println!("{}", day_1_real_answer)
}

fn solve_day_1(contents: String) -> i32 {
    let split = contents.split("\r\n");
    let iterator = split.into_iter();
    let mut dial = 50;
    let mut passes_of_0 = 0;
    iterator.for_each(|line| {
        let instruction = line.split_at(1);
        let direction = instruction.0;
        let amount = instruction.1.parse::<i32>().unwrap();
        if direction == "L" {
            dial += amount;
        } else {
            dial -= amount;
        }
        if dial > 99 {
            dial = dial % 100
        }
        while dial < 0 {
            dial += 100
        }
        if dial == 0 {
            passes_of_0 += 1;
        }

    });
    // println!("{}", iterator.next().unwrap());
    passes_of_0
}

fn fetch_input(day: i32, test: bool) -> String {
    let test_or_input = if test { "test" } else { "input" };
    let path = format!("input/day{}_{}.txt", day, test_or_input);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    contents
}
