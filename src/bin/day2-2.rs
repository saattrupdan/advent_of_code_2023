// As you continue your walk, the Elf poses a second question: in each game you played,
// what is the fewest number of cubes of each color that could have been in the bag to
// make the game possible?
//
// Again consider the example games from earlier:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue
// cubes. If any color had even one fewer cube, the game would have been impossible.
// Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
// Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
// Game 4 required at least 14 red, 3 green, and 15 blue cubes.
// Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
// The power of a set of cubes is equal to the numbers of red, green, and blue cubes
// multiplied together. The power of the minimum set of cubes in game 1 is 48. In games
// 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces
// the sum 2286.
//
// For each game, find the minimum set of cubes that must have been present. What is
// the sum of the power of these sets?

use std::collections::HashMap;

fn main() {
    // Read in the data file
    let data = include_str!("../../data/day2.txt");

    let mut max_cubes: HashMap<&str, i32> = HashMap::new();

    // Loop through each line of the data file
    let mut answer = 0;
    for line in data.lines() {
        let game = line.split(":").last().unwrap();
        max_cubes.clear();
        for draw in game.split(";") {
            for cube in draw.split(",") {
                let cube_parts: Vec<&str> = cube.trim().split(" ").collect();
                let cube_count = cube_parts[0].parse::<i32>().unwrap();
                let cube_color = cube_parts[1];
                if !max_cubes.contains_key(cube_color) {
                    max_cubes.insert(cube_color, cube_count);
                } else if cube_count > max_cubes[cube_color] {
                    max_cubes.insert(cube_color, cube_count);
                }
            }
        }
        let mut power = 1;
        for cube_count in max_cubes.values() {
            power *= cube_count;
        }
        answer += power;
    }

    println!("The answer for the first task is: {}", answer);
}
