// --- Part Two ---
// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You
// had the camel follow the instructions, but you've barely left your starting
// position. It's going to take significantly more steps to escape!
//
// What if the map isn't for people - what if the map is for ghosts? Are ghosts even
// bound by the laws of spacetime? Only one way to find out.
//
// After examining the maps a bit longer, your attention is drawn to a curious fact:
// the number of nodes with names ending in A is equal to the number ending in Z! If
// you were a ghost, you'd probably just start at every node that ends with A and
// follow all of the paths at the same time until they all simultaneously end up at
// nodes that end with Z.
//
// For example:
//
// LR
//
// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)
// Here, there are two starting nodes, 11A and 22A (because they both end with A). As
// you follow each left/right instruction, use that instruction to simultaneously
// navigate away from both nodes you're currently on. Repeat this process until all of
// the nodes you're currently on end with Z. (If only some of the nodes you're on end
// with Z, they act like any other node and you continue as normal.) In this example,
// you would proceed as follows:
//
// Step 0: You are at 11A and 22A.
// Step 1: You choose all of the left paths, leading you to 11B and 22B.
// Step 2: You choose all of the right paths, leading you to 11Z and 22C.
// Step 3: You choose all of the left paths, leading you to 11B and 22Z.
// Step 4: You choose all of the right paths, leading you to 11Z and 22B.
// Step 5: You choose all of the left paths, leading you to 11B and 22C.
// Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
//
// Simultaneously start on every node that ends with A. How many steps does it take
// before you're only on nodes that end with Z?

use num::integer::lcm;
use std::{collections::HashMap, str::Split};

fn main() {
    let data = include_str!("../../data/day8.txt");

    let mut split: Split<&str>;
    let mut key: &str;
    let mut value: Vec<&str>;
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    // Parse data
    let instructions = data.lines().next().unwrap();
    for line in data.lines().skip(2) {
        split = line.split(" = ");
        key = split.next().unwrap();
        value = split
            .next()
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect::<Vec<_>>();
        map.insert(key, value);
    }

    // Get initial set of active nodes
    let initial_nodes: Vec<&str> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| *k)
        .collect::<Vec<&str>>();

    let mut num_steps;
    let mut node: &str;
    let mut num_steps_map: HashMap<&str, usize> = HashMap::new();

    for initial_node in initial_nodes.into_iter() {
        node = initial_node;
        num_steps = 0;
        for instruction in instructions.chars().cycle() {
            match instruction {
                'L' => node = map.get(node).unwrap()[0],
                'R' => node = map.get(node).unwrap()[1],
                _ => panic!("Unknown instruction"),
            }
            num_steps += 1;
            if node.ends_with('Z') {
                num_steps_map.insert(initial_node, num_steps);
                break;
            }
        }
    }

    // The answer is the least common multiple of all the steps
    let answer = num_steps_map.values().fold(1, |acc, &x| lcm(acc, x));

    println!("The answer for the second task is: {}", answer);
}
