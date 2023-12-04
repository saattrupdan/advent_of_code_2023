// Your calculation isn't quite right. It looks like some of the digits are actually
// spelled out with letters: one, two, three, four, five, six, seven, eight, and nine
// also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last
// digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding
// these together produces 281.
//
// What is the sum of all of the calibration values?

use regex::Regex;

fn main() {
    let data = include_str!("../../data/day1.txt");

    // Regex that checks if a string starts with a digit or a digit word
    let re = Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // Loop through each line of the data file
    let mut answer = 0;
    for line in data.lines() {
        let mut first_digit = -1;
        let mut last_digit = -1;

        // Loop over a range of numbers from 0 to the length of the line
        for index in 0..line.len() {
            let truncated_line: &str = &line[index..];

            if re.is_match(truncated_line) {
                // Extract matched string
                let matched_str = re.find(truncated_line).unwrap().as_str();

                // Replace the matched string with the digit
                let replaced_str = matched_str
                    .replace("one", "1")
                    .replace("two", "2")
                    .replace("three", "3")
                    .replace("four", "4")
                    .replace("five", "5")
                    .replace("six", "6")
                    .replace("seven", "7")
                    .replace("eight", "8")
                    .replace("nine", "9");

                let found_digit = replaced_str.parse::<i32>().unwrap();

                if first_digit == -1 {
                    first_digit = found_digit;
                }
                last_digit = found_digit;
            }
        }
        let number = first_digit * 10 + last_digit;
        answer += number;
    }

    println!("The answer for the second task is: {}", answer);
}
