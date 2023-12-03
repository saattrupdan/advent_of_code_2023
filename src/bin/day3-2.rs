// The missing part wasn't the only issue - one of the gears in the engine is wrong. A
// gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is
// the result of multiplying those two numbers together.
//
// This time, you need to find the gear ratio of every gear and add them all up so that
// the engineer can figure out which gear needs to be replaced.
//
// Consider the same engine schematic again:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, there are two gears. The first is in the top left; it has part
// numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower
// right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is
// only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
//
// What is the sum of all of the gear ratios in your engine schematic?

use regex::Regex;


fn main() {
    // Read in the data file
    let data = include_str!("../../data/day3.txt");

    // Create a regex to match the special characters
    let gear_regex = Regex::new(r"[*]").unwrap();
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    let mut prev_line: &str;
    let mut next_line: &str;
    let mut number_matches: Vec<i32> = Vec::new();
    let mut answer: i32 = 0;

    for (line_idx, line) in data.lines().enumerate() {

        // Get the previous and next lines
        if line_idx == 0 {
            prev_line = ""
        } else {
            prev_line = data.lines().nth(line_idx - 1).unwrap();
        }
        next_line = data.lines().nth(line_idx + 1).unwrap_or("");

        for gear_match in gear_regex.find_iter(line) {
            number_matches.clear();

            for some_line in [prev_line, line, next_line].iter() {
                for number_match in number_regex.find_iter(some_line) {
                    if gear_match.start() + 1 >= number_match.start()
                        && gear_match.start() - 1 < number_match.end() {
                        number_matches.push(
                            number_match.as_str().parse::<i32>().unwrap()
                        );
                    }
                }
            }

            if number_matches.len() == 2 {
                answer += number_matches[0] * number_matches[1];
            }

        }
    }

    println!("The answer for the second task is: {}", answer);
}
