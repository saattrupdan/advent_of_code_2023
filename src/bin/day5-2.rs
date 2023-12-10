// --- Part Two ---
// Everyone will starve if you only plant such a small number of seeds. Re-reading the
// almanac, it looks like the seeds: line actually describes ranges of seed numbers.
//
// The values on the initial seeds: line come in pairs. Within each pair, the first
// value is the start of the range and the second value is the length of the range. So,
// in the first line of the example above:
//
// seeds: 79 14 55 13
// This line describes two ranges of seed numbers to be planted in the garden. The
// first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92.
// The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66,
// 67.
//
// Now, rather than considering four seed numbers, you need to consider a total of 27
// seed numbers.
//
// In the above example, the lowest location number can be obtained from seed number
// 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45,
// humidity 46, and location 46. So, the lowest location number is 46.
//
// Consider all of the initial seed numbers listed in the ranges on the first line of
// the almanac. What is the lowest location number that corresponds to any of the
// initial seed numbers?

use regex::Regex;
use std::collections::HashMap;

// Doesn't work yet
fn main() {
    // let data = include_str!("../../data/day5.txt");

    // let map_regex = Regex::new(r"^[a-z]+-to-[a-z]+ map:$").unwrap();
    // let types = vec![
    //     "seed",
    //     "soil",
    //     "fertilizer",
    //     "water",
    //     "light",
    //     "temperature",
    //     "humidity",
    //     "location",
    // ];
    // let mut id_ranges: Vec<(u64, u64)> = Vec::new();
    // let mut old_id_ranges: Vec<(u64, u64)>;
    // let mut all_maps: HashMap<&str, Vec<Vec<u64>>> = HashMap::new();
    // let mut active_map_name: Option<&str> = None;
    // let mut active_map: Vec<u64>;

    // // Parse all the data
    // for line in data.lines() {
    //     if line.starts_with("seeds:") {
    //         id_ranges = line
    //             .split(":")
    //             .last()
    //             .unwrap()
    //             .trim()
    //             .split(" ")
    //             .map(|x| x.parse::<u64>().unwrap())
    //             .collect::<Vec<u64>>()
    //             .chunks(2)
    //             .map(|x| (x[0], x[0] + x[1]))
    //             .collect::<Vec<(u64, u64)>>();
    //     } else if map_regex.is_match(line) {
    //         active_map_name = Some(line.split(" ").nth(0).unwrap());
    //     } else if active_map_name.is_some() && line == "" {
    //         active_map_name = None;
    //     } else if active_map_name.is_some() {
    //         active_map = line
    //             .trim()
    //             .split(" ")
    //             .map(|x| x.parse::<u64>().unwrap())
    //             .collect::<Vec<u64>>();
    //         all_maps
    //             .entry(active_map_name.unwrap())
    //             .and_modify(|x| x.push(active_map.clone()))
    //             .or_insert(vec![active_map.clone()]);
    //     }
    // }

    // // Get the final location IDs by mapping the IDs through all the maps
    // let mut remainder_ranges = Vec::new();
    // for type_id in 0..types.len() - 1 {
    //     old_id_ranges = id_ranges.clone();
    //     id_ranges.clear();
    //     for (old_id_start, old_id_end) in old_id_ranges.iter() {
    //         let map_name = format!("{}-to-{}", types[type_id], types[type_id + 1]);
    //         for map_line in all_maps.get(map_name.as_str()).unwrap() {
    //             let mapping_target_start = map_line[1];
    //             let mapping_source_start = map_line[0];
    //             let mapping_range = map_line[2];

    //             if old_id_end > &mapping_source_start
    //                 && old_id_start < &(mapping_source_start + mapping_range)
    //             {
    //                 let id_start = old_id_start.max(&mapping_source_start) - mapping_source_start
    //                     + mapping_target_start;
    //                 let id_end = old_id_end.min(&(mapping_source_start + mapping_range))
    //                     - mapping_source_start
    //                     + mapping_target_start;
    //                 id_ranges.push((id_start, id_end));

    //                 if old_id_start < &mapping_source_start {
    //                     remainder_ranges.push((old_id_start, mapping_source_start));
    //                 }
    //                 if old_id_end > &(mapping_source_start + mapping_range) {
    //                     remainder_ranges
    //                         .push((&(mapping_source_start + mapping_range), old_id_end.clone()));
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("Remainder ranges: {:?}", remainder_ranges);

    // let answer = id_ranges.iter().map(|x| x.0).min().unwrap();
    // println!("The answer for the second task is: {}", answer);
}
