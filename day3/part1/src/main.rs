use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").expect("Error while reading the file");

    let joltages: i32 = binding
        .lines()
        .map(|line| {
            let digits = line
                .split("")
                .filter(|character| !character.is_empty())
                .map(|character| character.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();

            let digits_except_last: Vec<i32> =
                digits.clone().into_iter().take(digits.len() - 1).collect();

            let largest_digit = digits_except_last.iter().max().copied().unwrap_or_default();

            let largest_digit_index = digits_except_last
                .clone()
                .into_iter()
                .position(|digit| digit == largest_digit)
                .unwrap_or(0);

            let remaining_digits: Vec<i32> = digits
                .clone()
                .into_iter()
                .skip(largest_digit_index + 1)
                .collect();

            let largest_remaining_digit = remaining_digits
                .clone()
                .into_iter()
                .max()
                .unwrap_or_default();

            largest_digit * 10 + largest_remaining_digit
        })
        .sum();

    println!("Joltages are {:?}", joltages);
}
