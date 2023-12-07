// --- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a
// look. The Elves have even given you a map; on it, they've used stars to mark the top
// fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need
// to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in
// the Advent calendar; the second puzzle is unlocked when you complete the first. Each
// puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and
// where they're even sending you ("the sky") and why your map looks mostly blank ("you
// sure ask a lot of questions") and hang on did you just say the sky ("of course,
// where do you think snow comes from") when you realize that the Elves are already
// loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration
// document (your puzzle input) has been amended by a very young Elf who was apparently
// just excited to show off her art skills. Consequently, the Elves are having trouble
// reading the values on the document.
//
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

        // Add the number to the total
        answer += number;
    }

    println!("The answer for the first task is: {}", answer);
}
