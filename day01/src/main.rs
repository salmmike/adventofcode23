use std::env;
use std::fs::read_to_string;
use std::option::Option;

fn string_to_num(chars: &[u8]) -> Option<u8> {
    if chars.len() < 3 {
        return None;
    }
    if chars.starts_with("one".as_bytes()) {
        return Some(1);
    }
    if chars.starts_with("two".as_bytes()) {
        return Some(2);
    }
    if chars.starts_with("three".as_bytes()) {
        return Some(3);
    }
    if chars.starts_with("four".as_bytes()) {
        return Some(4);
    }
    if chars.starts_with("five".as_bytes()) {
        return Some(5);
    }
    if chars.starts_with("six".as_bytes()) {
        return Some(6);
    }
    if chars.starts_with("seven".as_bytes()) {
        return Some(7);
    }
    if chars.starts_with("eight".as_bytes()) {
        return Some(8);
    }
    if chars.starts_with("nine".as_bytes()) {
        return Some(9);
    }
    None
}

/// Get list of digits in a string in the order they appear
fn get_digits(str: &String) -> Vec<u8> {
    const RADIX: u32 = 10;
    let mut retval: Vec<u8> = Vec::new();

    for (i, c) in str.chars().enumerate() {
        if (c >= '0') & (c <= '9') {
            retval.push(
                c.to_digit(RADIX).unwrap() as u8
            );
        } else {
            let val: Option<u8> = string_to_num(&str.as_bytes()[i..]);
            if val.is_some() {
                retval.push(val.unwrap());
            }
        }
    }
    retval
}

fn join_first_and_last(values: &Vec<u8>) -> u32 {
    if values.is_empty() {
        return 0;
    }
    (values[0] as u32) * 10 + values[values.len() -1] as u32
}

/// Read file filename into a vector, with each line as on element.
fn parse_inputfile(filename: &String) -> Vec<String> {
    if filename.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
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

fn main() {
    let filename: String = get_filename();
    let input_vec: Vec<String> = parse_inputfile(&filename);
    let mut sum: u32 = 0;
    for line in input_vec {
        let digits: Vec<u8> = get_digits(&line);
        sum += join_first_and_last(&digits);
    }

    println!("Sum is {}", sum);
}
