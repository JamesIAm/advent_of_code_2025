use crate::file_utils::fetch_input;

const DAY: i32 = 3;
const TEST_ANSWER: usize = 3121910778619;

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
        let bank_nums: Vec<usize> = convert_to_individual_numbers(bank);
        let mut final_battery = String::new();
        let mut rest_of_list = bank_nums;
        for i in (0..12).rev() {
            let next_battery = get_next_largest_battery(&mut rest_of_list, i);
            let position_of_battery = rest_of_list.iter().position(|x| x == &next_battery).unwrap();
            rest_of_list = rest_of_list.split_off(position_of_battery + 1);
            final_battery += &*next_battery.to_string();
            // println!("next {}", next_battery);
        }
        // println!("final {}", final_battery);
        strongest_batteries.push(final_battery.parse().unwrap());
    }
    strongest_batteries.iter().fold(0, |acc, x| acc + x)
}

fn get_next_largest_battery(bank_nums: &mut Vec<usize>, numbers_left_to_find: usize) -> usize {
    let mut sorted_batteries: Vec<usize> =
        bank_nums.clone().split_at(bank_nums.len() - (numbers_left_to_find)).0.to_vec();
    sort_descending(&mut sorted_batteries);
    let next_battery = sorted_batteries[0];
    next_battery
}

fn convert_to_individual_numbers(bank: &str) -> Vec<usize> {
    bank.chars()
        .into_iter()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn sort_descending(list: &mut Vec<usize>) {
    list.sort_by(|a, b| b.cmp(a));
}
