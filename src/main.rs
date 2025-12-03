use std::io;

mod day1;
mod day2;
mod day3;
mod file_utils;

fn main() {
    day3::main();
    let buffer = &mut String::new();
    println!("Enter \"y\" to run old days");
    io::stdin().read_line(buffer).expect("TODO: panic message");
    if buffer == "y" {
        old_days();
    }
}
fn old_days() {
    day1::main();
    day2::main();
}