// --- Part Two ---
// Of course, it would be nice to have even more history included in your report.
// Surely it's safe to just extrapolate backwards as well, right?
//
// For each history, repeat the process of finding differences until the sequence of
// differences is entirely zero. Then, rather than adding a zero to the end and filling
// in the next values of each previous sequence, you should instead add a zero to the
// beginning of your sequence of zeroes, then fill in new first values for each
// previous sequence.
//
// In particular, here is what the third example history looks like when extrapolating
// back in time:
//
// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2   0   2   4   6
//       2   2   2   2
//         0   0   0
// Adding the new values on the left side of each sequence from bottom to top
// eventually reveals the new left-most history value: 5.
//
// Doing this for the remaining example data above results in previous values of -3 for
// the first history and 0 for the second history. Adding all three new values together
// produces 2.
//
// Analyze your OASIS report again, this time extrapolating the previous value for each
// history. What is the sum of these extrapolated values?

fn main() {
    let data = include_str!("../../data/day9.txt");

    // Parse data
    let histories = data
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .into_iter();

    let mut residuals: Vec<i64>;
    let mut first_residuals: Vec<i64> = Vec::new();
    let mut extrapolated: i64;
    let mut answer = 0;

    for history in histories {
        residuals = history.clone();
        first_residuals.clear();
        extrapolated = 0;
        while residuals.iter().any(|&n| n != 0) {
            first_residuals.push(residuals[0]);
            residuals = residuals
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<i64>>();
        }
        for residual in first_residuals.iter().rev() {
            extrapolated = residual - extrapolated;
        }
        answer += extrapolated;
    }

    println!("The answer for the second task is: {}", answer);
}
