use crate::file_utils::fetch_input;

const DAY: i32 = 3;
const TEST_ANSWER: usize = 357;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}

fn solve_day(contents: String) -> usize {
    let battery_banks: Vec<&str> = contents.split("\r\n").collect();
    let mut strongest_batteries: Vec<usize> = Vec::new();
    for bank in battery_banks {
        let mut bank_nums: Vec<usize> = bank
            .chars()
            .into_iter()
            .map(|x| x.to_string().parse().unwrap())
            .collect();
        let mut sorted_batteries: Vec<usize> = bank_nums.clone();
        sort_descending(&mut sorted_batteries);
        let final_battery = bank_nums.last().unwrap();
        let battery1 = if *final_battery == sorted_batteries[0] {
            sorted_batteries[1]
        } else {
            sorted_batteries[0]
        };
        let position_of_battery_1 = bank_nums.iter().position(|x| x == &battery1).unwrap();
        let mut rest_of_list: Vec<usize> = bank_nums.split_off(position_of_battery_1+1);
        sort_descending(&mut rest_of_list);
        let battery2 = rest_of_list[0];
        let total_battery: usize = (battery1.to_string() + &*battery2.to_string())
            .parse()
            .unwrap();
        strongest_batteries.push(total_battery);
    }
    let total = strongest_batteries.iter().fold(0, |acc, x| acc + x);
    total
}

fn sort_descending(list: &mut Vec<usize>) {
    list.sort_by(|a, b| b.cmp(a));
}
