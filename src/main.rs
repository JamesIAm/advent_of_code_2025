use std::io;

mod day1;
mod day2;
mod day3;
mod file_utils;
mod day4;
mod day5;

fn main() {
    day5::main();
    run_old_days();
}

fn run_old_days() {
    let buffer = &mut String::new();
    println!("Enter \"y\" to run old days");
    io::stdin().read_line(buffer).expect("TODO: panic message");
    if buffer == "y\r\n" {
        old_days();
    }
}

fn old_days() {
    day1::main();
    day2::main();
    day3::main();
    day4::main();
    day5::main();
}