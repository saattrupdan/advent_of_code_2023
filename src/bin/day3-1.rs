// The engineer explains that an engine part seems to be missing from the engine, but
// nobody can figure out which one. If you can add up all the part numbers in the
// engine schematic, it should be easy to work out which part is missing.
//
// The engine schematic (your puzzle input) consists of a visual representation of the
// engine. There are lots of numbers and symbols you don't really understand, but
// apparently any number adjacent to a symbol, even diagonally, is a "part number" and
// should be included in your sum. (Periods (.) do not count as a symbol.)
//
// Here is an example engine schematic:
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
// In this schematic, two numbers are not part numbers because they are not adjacent to
// a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a
// symbol and so is a part number; their sum is 4361.
//
// Of course, the actual engine schematic is much larger. What is the sum of all of the
// part numbers in the engine schematic?

use regex::Regex;

fn main() {
    // Read in the data file
    let data = include_str!("../../data/day3.txt");

    // Create a regex to match the special characters
    let special_character_regex = Regex::new(r"[^0-9.]").unwrap();
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    let mut prev_line: &str;
    let mut next_line: &str;
    let mut prev_symbol_indices: Vec<usize>;
    let mut current_symbol_indices: Vec<usize>;
    let mut next_symbol_indices: Vec<usize>;
    let mut indices: Vec<usize> = Vec::new();
    let mut answer: i32 = 0;

    for (line_idx, line) in data.lines().enumerate() {
        indices.clear();

        // Get the previous and next lines
        if line_idx == 0 {
            prev_line = ""
        } else {
            prev_line = data.lines().nth(line_idx - 1).unwrap();
        }
        next_line = data.lines().nth(line_idx + 1).unwrap_or("");

        // Get the indices of all the special characters in the three lines
        prev_symbol_indices = special_character_regex
            .find_iter(prev_line)
            .map(|m| m.start())
            .collect();
        current_symbol_indices = special_character_regex
            .find_iter(line)
            .map(|m| m.start())
            .collect();
        next_symbol_indices = special_character_regex
            .find_iter(next_line)
            .map(|m| m.start())
            .collect();

        // Get the indices of all characters in the current line which are adjacent to a
        // special character on any of the three lines, including diagonally
        indices.extend(prev_symbol_indices.iter().map(|&i| i - 1));
        indices.extend(prev_symbol_indices.iter());
        indices.extend(prev_symbol_indices.iter().map(|&i| i + 1));
        indices.extend(current_symbol_indices.iter().map(|&i| i - 1));
        indices.extend(current_symbol_indices.iter().map(|&i| i + 1));
        indices.extend(next_symbol_indices.iter().map(|&i| i - 1));
        indices.extend(next_symbol_indices.iter());
        indices.extend(next_symbol_indices.iter().map(|&i| i + 1));

        for number_match in number_regex.find_iter(line) {
            // Check if any of the `indices` occur within the range of the current number
            let indices_in_number: Vec<usize> = indices
                .iter()
                .filter(|&&i| i >= number_match.start() && i < number_match.end())
                .map(|&i| i)
                .collect();

            // If there are any indices in the number, add the number to the answer
            if indices_in_number.len() > 0 {
                answer += number_match.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("The answer for the first task is: {}", answer);
}
