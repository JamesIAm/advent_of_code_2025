use ::std::fs;

pub fn fetch_input(day: i32, test: bool) -> String {
    let test_or_input = if test { "test" } else { "input" };
    let path = format!("input/DAY{}_{}.txt", day, test_or_input);
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents
}
