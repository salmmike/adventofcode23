use std::env;
use std::fs::read_to_string;

/// INPUT PARSES

/// Read file filename into a vector, with each line as on element.
fn parse_inputfile(filename: &String) -> Vec<String> {
    if filename.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        if !line.is_empty() {
            result.push(line.to_string().trim().to_string());
        }
    }
    result
}

fn get_filename() -> String {
    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            return arg.to_string();
        }
    }
    String::new()
}

/// Calculate all possible times that reach distance
// x = t*v
// v = tv
// x = (t - v) * v
// x = t*v - v*v
fn press_times_for_distance(total_time: u64, distance: u64) -> u64 {
    let mut press_times: u64 = 0;

    for v in 1..total_time - 1 {
        if (total_time - v) * v > distance {
            press_times += 1;
        }
    }

    press_times
}

fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut long_dist_string = String::new();
    let mut long_time_string = String::new();

    for t in input[0].split(':').nth(1).unwrap().split_whitespace() {
        times.push(t.to_string().trim().parse().unwrap());
        long_time_string += t.to_string().trim();
    }

    for d in input[1].split(':').nth(1).unwrap().split_whitespace() {
        distances.push(d.to_string().trim().parse().unwrap());
        long_dist_string += d.to_string().trim();
    }

    let mut output: u64 = 1;

    for i in 0..times.len() {
        output *= press_times_for_distance(times[i], distances[i]);
    }

    println!("{}", output);

    let long_time: u64 = long_time_string.parse().unwrap();
    let long_dist: u64 = long_dist_string.parse().unwrap();

    println!("long time {}", long_time);
    println!("long dist {}", long_dist);

    let mut output: u64 = 1;

    output *= press_times_for_distance(long_time, long_dist);
    println!("{}", output);
}
