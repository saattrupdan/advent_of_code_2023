// --- Part Two ---
// You quickly reach the farthest point of the loop, but the animal never emerges.
// Maybe its nest is within the area enclosed by the loop?
//
// To determine whether it's even worth taking the time to search for such a nest, you
// should calculate how many tiles are contained within the loop. For example:
//
// ...........
// .S-------7.
// .|F-----7|.
// .||.....||.
// .||.....||.
// .|L-7.F-J|.
// .|..|.|..|.
// .L--J.L--J.
// ...........
// The above loop encloses merely four tiles - the two pairs of . in the southwest and
// southeast (marked I below). The middle . tiles (marked O below) are not in the loop.
// Here is the same loop again with those regions marked:
//
// ...........
// .S-------7.
// .|F-----7|.
// .||OOOOO||.
// .||OOOOO||.
// .|L-7OF-J|.
// .|II|O|II|.
// .L--JOL--J.
// .....O.....
// In fact, there doesn't even need to be a full tile path to the outside for tiles to
// count as outside the loop - squeezing between pipes is also allowed! Here, I is
// still within the loop and O is still outside the loop:
//
// ..........
// .S------7.
// .|F----7|.
// .||OOOO||.
// .||OOOO||.
// .|L-7F-J|.
// .|II||II|.
// .L--JL--J.
// ..........
// In both of the above examples, 4 tiles are enclosed by the loop.
//
// Here's a larger example:
//
// .F----7F7F7F7F-7....
// .|F--7||||||||FJ....
// .||.FJ||||||||L7....
// FJL7L7LJLJ||LJ.L-7..
// L--J.L7...LJS7F-7L7.
// ....F-J..F7FJ|L7L7L7
// ....L7.F7||L7|.L7L7|
// .....|FJLJ|FJ|F7|.LJ
// ....FJL-7.||.||||...
// ....L---J.LJ.LJLJ...
// The above sketch has many random bits of ground, some of which are in the loop (I)
// and some of which are outside it (O):
//
// OF----7F7F7F7F-7OOOO
// O|F--7||||||||FJOOOO
// O||OFJ||||||||L7OOOO
// FJL7L7LJLJ||LJIL-7OO
// L--JOL7IIILJS7F-7L7O
// OOOOF-JIIF7FJ|L7L7L7
// OOOOL7IF7||L7|IL7L7|
// OOOOO|FJLJ|FJ|F7|OLJ
// OOOOFJL-7O||O||||OOO
// OOOOL---JOLJOLJLJOOO
// In this larger example, 8 tiles are enclosed by the loop.
//
// Any tile that isn't part of the main loop can count as being enclosed by the loop.
// Here's another example with many bits of junk pipe lying around that aren't
// connected to the main loop at all:
//
// FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L
// Here are just the tiles that are enclosed by the loop marked with I:
//
// FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJIF7FJ-
// L---JF-JLJIIIIFJLJJ7
// |F|F-JF---7IIIL7L|7|
// |FFJF7L7F-JF7IIL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L
// In this last example, 10 tiles are enclosed by the loop.
//
// Figure out whether you have time to search for the nest by calculating the area
// within the loop. How many tiles are enclosed by the loop?

use std::collections::HashMap;

// TODO: This currently doesn't give the correct solution. At least one thing wrong
// with it is my assumption that if you have an odd number of giant loop tiles in a
// given direction, then the tile is enclosed by the giant loop. This is not true, so
// instead I could try keeping track of the direction as we're forming the giant loop,
// and keep marking the tile to the right of our direction as inner/outer, and counting
// those at the end.
fn main() {
    let data = include_str!("../../data/day10.txt");

    // Parse data as a matrix
    let matrix = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Find the coordinates of the starting point
    let mut start = (0, 0);
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (i, j);
            }
        }
    }

    let mut valid_neighbour_map: HashMap<usize, Vec<(usize, usize)>>;
    let mut paths: HashMap<usize, Vec<(usize, usize)>> = {
        let mut paths: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        paths.insert(0, vec![start]);
        paths
    };
    let mut previous: Option<(usize, usize)>;
    let mut new_path_idx = 1;

    while !paths
        .values()
        .any(|path| path.last().is_some() && path.last().unwrap() == &start && path.len() > 1)
    {
        valid_neighbour_map = HashMap::new();
        for (path_idx, path) in paths.iter() {
            if path.len() > 1 {
                previous = Some(path[path.len() - 2]);
            } else {
                previous = None;
            }
            let current: (usize, usize) = path[path.len() - 1];

            // Determine the directions that the current node can be connected
            let can_be_connected_north = current.0 > 0
                && ['S', 'F', '7', '|'].contains(&matrix[current.0 - 1][current.1])
                && ['S', 'J', 'L', '|'].contains(&matrix[current.0][current.1]);
            let can_be_connected_east = current.1 < matrix[0].len() - 1
                && ['S', 'J', '7', '-'].contains(&matrix[current.0][current.1 + 1])
                && ['S', 'F', 'L', '-'].contains(&matrix[current.0][current.1]);
            let can_be_connected_south = current.0 < matrix.len() - 1
                && ['S', 'L', 'J', '|'].contains(&matrix[current.0 + 1][current.1])
                && ['S', 'F', '7', '|'].contains(&matrix[current.0][current.1]);
            let can_be_connected_west = current.1 > 0
                && ['S', 'L', 'F', '-'].contains(&matrix[current.0][current.1 - 1])
                && ['S', 'J', '7', '-'].contains(&matrix[current.0][current.1]);

            // Find the valid neighbours
            let mut valid_neighbours: Vec<(usize, usize)> = Vec::new();
            if can_be_connected_north {
                valid_neighbours.push((current.0 - 1, current.1));
            }
            if can_be_connected_east {
                valid_neighbours.push((current.0, current.1 + 1));
            }
            if can_be_connected_south {
                valid_neighbours.push((current.0 + 1, current.1));
            }
            if can_be_connected_west {
                valid_neighbours.push((current.0, current.1 - 1));
            }

            // Ensure that the previous node is not in the valid neighbours
            if previous.is_some() {
                if valid_neighbours.contains(&previous.unwrap()) {
                    valid_neighbours.retain(|&n| n != previous.unwrap());
                }
            }

            // Add the valid neighbours to the map, if any
            if valid_neighbours.len() > 0 {
                valid_neighbour_map.insert(*path_idx, valid_neighbours);
            }
        }

        // For each valid neighbour, create a new path
        for (path_idx, valid_neighbours) in valid_neighbour_map.iter() {
            let old_path: Vec<(usize, usize)> = paths[path_idx].clone();
            for valid_neighbour in valid_neighbours.iter() {
                let mut new_path = old_path.clone();
                new_path.push(*valid_neighbour);
                new_path_idx += 1;
                paths.insert(new_path_idx, new_path);
            }
            paths.remove(path_idx);
        }
    }

    let giant_loop = paths
        .values()
        .find(|path| path.last().is_some() && path.last().unwrap() == &start && path.len() > 1)
        .unwrap();

    // Remove last element, which is the starting point
    let giant_loop = &giant_loop[..giant_loop.len() - 1];

    println!("Length of the giant loop: {}", giant_loop.len());
    println!("Total amount of tiles: {}", matrix.len() * matrix[0].len());

    // Get the giant loop's bounding box
    let mut max_north: usize = 0;
    let mut max_east: usize = matrix[0].len();
    let mut max_south: usize = matrix.len();
    let mut max_west: usize = 0;
    for (i, j) in giant_loop.iter() {
        if *i < max_north {
            max_north = *i;
        }
        if *j > max_east {
            max_east = *j;
        }
        if *i > max_south {
            max_south = *i;
        }
        if *j < max_west {
            max_west = *j;
        }
    }

    let mut answer: usize = 0;
    let mut has_north_giant_loop_tile: bool;
    let mut has_east_giant_loop_tile: bool;
    let mut has_south_giant_loop_tile: bool;
    let mut has_west_giant_loop_tile: bool;

    println!(
        "Max boundaries: North: {max_north}, South: {max_south}, West: {max_west}, East: {max_east}",
    );

    // To find the tiles enclosed by the loop, we iterate over all the tiles in the
    // matrix, and check if there exist "giant loop tiles" in all four directions from
    // the given tile. If so, we add the tile to the enclosed tiles.
    for i in max_north..max_south {
        println!(
            "{current} / {maximum}",
            current = i - max_north,
            maximum = max_south - max_north
        );

        for j in max_west..max_east {
            // Enclosed tiles cannot be giant loop tiles
            if giant_loop.contains(&(i, j)) {
                continue;
            }

            // Check if the tile has an odd number of giant loop tiles in the northern
            // direction, since this would mean that the tile is enclosed by the giant
            // loop. If it doesn't then it cannot be an enclosed tile, and we skip it.
            let mut num_north_giant_loop_tiles = 0;
            for row_idx in max_north..i {
                if giant_loop.contains(&(row_idx, j)) {
                    num_north_giant_loop_tiles += 1;
                }
            }
            has_north_giant_loop_tile = num_north_giant_loop_tiles % 2 == 1;
            if !has_north_giant_loop_tile {
                continue;
            }

            // Check if the tile has an odd number of giant loop tiles in the western
            // direction, since this would mean that the tile is enclosed by the giant
            // loop. If it doesn't then it cannot be an enclosed tile, and we skip it.
            let mut num_west_giant_loop_tiles = 0;
            for col_idx in max_west..j {
                if giant_loop.contains(&(i, col_idx)) {
                    num_west_giant_loop_tiles += 1;
                }
            }
            has_west_giant_loop_tile = num_west_giant_loop_tiles % 2 == 1;
            if !has_west_giant_loop_tile {
                continue;
            }

            // Check if the tile has an odd number of giant loop tiles in the eastern
            // direction, since this would mean that the tile is enclosed by the giant
            // loop. If it doesn't then it cannot be an enclosed tile, and we skip it.
            let mut num_east_giant_loop_tiles = 0;
            for col_idx in j + 1..max_east {
                if giant_loop.contains(&(i, col_idx)) {
                    num_east_giant_loop_tiles += 1;
                }
            }
            has_east_giant_loop_tile = num_east_giant_loop_tiles % 2 == 1;
            if !has_east_giant_loop_tile {
                continue;
            }

            // Check if the tile has an odd number of giant loop tiles in the southern
            // direction, since this would mean that the tile is enclosed by the giant
            // loop. If it doesn't then it cannot be an enclosed tile, and we skip it.
            let mut num_south_giant_loop_tiles = 0;
            for row_idx in i + 1..max_south {
                if giant_loop.contains(&(row_idx, j)) {
                    num_south_giant_loop_tiles += 1;
                }
            }
            has_south_giant_loop_tile = num_south_giant_loop_tiles % 2 == 1;
            if !has_south_giant_loop_tile {
                continue;
            }

            // If we reach this point, then the tile is enclosed by the giant loop, and
            // we add it to the answer.
            answer += 1;
        }
    }

    println!("The answer for the second task is: {}", answer);
}
