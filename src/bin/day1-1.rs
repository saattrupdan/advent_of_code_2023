// The newly-improved calibration document consists of lines of text; each line
// originally contained a specific calibration value that the Elves now need to
// recover. On each line, the calibration value can be found by combining the first
// digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77.
// Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration
// values?

use std::str::Chars;

fn main() {
    // Read in the data file
    let data = include_str!("../../data/day1.txt");

    // Loop through each line of the data file
    let mut answer = 0;
    for line in data.lines() {

        // Convert the line to a vector of characters
        let characters: Chars = line.chars();

        // Loop through each character in the line
        let mut first_digit = 0;
        let mut last_digit = 0;
        for character in characters {
            if character.is_digit(10) {
                if first_digit == 0 {
                    first_digit = character.to_digit(10).unwrap();
                } else {
                    last_digit = character.to_digit(10).unwrap();
                }
            }
        }

        // If the first digit is set and the last one isn't, set it to the first digit
        if first_digit != 0 && last_digit == 0 {
            last_digit = first_digit;
        }

        // Concatenate the two digits together
        let number = first_digit * 10 + last_digit;

        answer += number;
    }

    println!("The answer for the first task is: {}", answer);
}
