// --- Part Two ---
// As the race is about to start, you realize the piece of paper with race times and
// record distances you got earlier actually just has very bad kerning. There's really
// only one race - ignore the spaces between the numbers on each line.
//
// So, the example from before:
//
// Time:      7  15   30
// Distance:  9  40  200
// ...now instead means this:
//
// Time:      71530
// Distance:  940200
// Now, you have to figure out how many ways there are to win this single race. In this
// example, the race lasts for 71530 milliseconds and the record distance you need to
// beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516
// milliseconds and beat the record, a total of 71503 ways!
//
// How many ways can you beat the record in this one much longer race?

fn main() {
    let data = include_str!("../../data/day6.txt");

    // Extract the race data
    let mut lines = data.lines();
    let race_durations: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter_map(|x| x.trim().parse::<u64>().ok())
        .collect();
    let record_distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter_map(|x| x.trim().parse::<u64>().ok())
        .collect();

    let mut distance: u64;
    let mut answer = 0;

    let race_duration = race_durations
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<u64>()
        .unwrap();
    let record_distance = record_distances
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<u64>()
        .unwrap();

    for charging_time in 0..race_duration {
        distance = (race_duration - charging_time) * charging_time;
        if distance > record_distance {
            answer += 1;
        }
    }

    println!("The answer for the second task is: {}", answer);
}
