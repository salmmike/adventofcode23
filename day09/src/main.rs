use std::env;
use std::fs::read_to_string;

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

fn next_value(input: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut res: Vec<i32> = Vec::new();
    if input.len() == 1 {
        return (input[0], input.clone());
    }

    for (i, val) in input.iter().enumerate() {
        res.push(input[i + 1] - val);

        if i == input.len() - 2 {
            break;
        }
    }
    let mut sum = 0;

    res.iter().for_each(|v| {
        sum += v;
    });

    (sum, res)
}

fn get_last(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    input.iter().for_each(|v: &Vec<i32>| {
        sum += v.last().unwrap();
    });

    sum
}

fn get_first(input: &Vec<Vec<i32>>) -> i32 {
    let mut last_new_first = 0;
    input.iter().rev().for_each(|v: &Vec<i32>| {
        last_new_first = v.first().unwrap() - last_new_first;
    });

    last_new_first
}

fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());

    let op_start: std::time::Instant = std::time::Instant::now();

    let mut inputs: Vec<Vec<i32>> = Vec::new();
    inputs.resize(input.len(), Vec::new());

    for (i, line) in input.iter().enumerate() {
        line.split_ascii_whitespace()
            .for_each(|s| inputs[i].push(s.to_string().parse().unwrap()));
    }
    println!("Parsing took: {:?}", op_start.elapsed());

    let op_start: std::time::Instant = std::time::Instant::now();

    let mut total_sum: i32 = 0;
    let mut total_first_sum: i32 = 0;

    for v in inputs.iter() {
        let mut input: Vec<i32> = v.clone();
        let mut input_sum = 1;
        let mut all_outputs: Vec<Vec<i32>> = Vec::new();
        all_outputs.push(input.clone());

        while input_sum != 0 {
            let res: (i32, Vec<i32>) = next_value(&input);
            input_sum = res.0;
            input = res.1;
            all_outputs.push(input.clone());
        }

        total_sum += get_last(&all_outputs);
        total_first_sum += get_first(&all_outputs);
    }

    println!(
        "\n{} {} {:?}",
        total_sum,
        total_first_sum,
        op_start.elapsed()
    );
}
