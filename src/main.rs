use std::io;

mod day1;
mod day2;
mod day3;
mod file_utils;
mod day4;

fn main() {
    let mut x : i32= 1;
    x -= 2;
    println!("{}", x as usize);
    day4::main();
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
    day3::main();
}