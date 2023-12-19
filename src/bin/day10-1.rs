// --- Day 10: Pipe Maze ---
// You use the hang glider to ride the hot air from Desert Island all the way up to the
// floating metal island. This island is surprisingly cold and there definitely aren't
// any thermals to glide on, so you leave your hang glider behind.
//
// You wander around for a while, but you don't find any people or animals. However,
// you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly
// consistent direction; maybe you can find someone at the hot springs and ask them
// where the desert-machine parts are made.
//
// The landscape here is alien; even the flowers and trees are made of metal. As you
// stop to admire some metal grass, you notice something metallic scurry away in your
// peripheral vision and jump into a big pipe! It didn't look like any animal you've
// ever seen; if you want a better look, you'll need to get ahead of it.
//
// Scanning the area, you discover that the entire field you're standing on is densely
// packed with pipes; it was hard to tell at first because they're the same metallic
// silver color as the "ground". You make a quick sketch of all of the surface pipes
// you can see (your puzzle input).
//
// The pipes are arranged in a two-dimensional grid of tiles:
//
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your
// sketch doesn't show what shape the pipe has.
// Based on the acoustics of the animal's scurrying, you're confident the pipe that
// contains the animal is one large, continuous loop.
//
// For example, here is a square loop of pipe:
//
// .....
// .F-7.
// .|.|.
// .L-J.
// .....
// If the animal had entered this loop in the northwest corner, the sketch would
// instead look like this:
//
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// In the above diagram, the S tile is still a 90-degree F bend: you can tell because
// of how the adjacent pipes connect to it.
//
// Unfortunately, there are also many pipes that aren't connected to the loop! This
// sketch shows the same loop as above:
//
// -L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF
// In the above diagram, you can still figure out which pipes form the main loop:
// they're the ones connected to S, pipes those pipes connect to, pipes those pipes
// connect to, and so on. Every pipe in the main loop connects to its two neighbors
// (including S, which will have exactly two pipes connecting to it, and which is
// assumed to connect back to those two pipes).
//
// Here is a sketch that contains a slightly more complex main loop:
//
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
//
// 7-F7-
// .FJ|7
// SJLL7
// |F--J
// LJ.LJ
// If you want to get out ahead of the animal, you should find the tile in the loop
// that is farthest from the starting position. Because the animal is in the pipe, it
// doesn't make sense to measure this by direct distance. Instead, you need to find the
// tile that would take the longest number of steps along the loop to reach from the
// starting point - regardless of which way around the loop the animal went.
//
// In the first example with the square loop:
//
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// You can count the distance each tile in the loop is from the starting point like
// this:
//
// .....
// .012.
// .1.3.
// .234.
// .....
// In this example, the farthest point from the start is 4 steps away.
//
// Here's the more complex loop again:
//
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here are the distances for each tile on that loop:
//
// ..45.
// .236.
// 01.78
// 14567
// 23...
// Find the single giant loop starting at S. How many steps along the loop does it take
// to get from the starting position to the point farthest from the starting position?

use std::collections::HashMap;

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

    // The answer is the half of the length of the giant loop, rounded down
    let answer = (giant_loop.len() as f32 / 2.0).floor();
    println!("The answer for the first task is: {}", answer);
}
