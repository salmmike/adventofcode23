use std::cmp::min;
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
            result.push(format!("{}", line.to_string().trim()));
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

/// Parse number and length of number from String beginning with a number.
fn parse_num(input: &String) -> u32 {
    input.trim().to_string().parse().unwrap()
}

fn get_winning(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut winning: Vec<u32> = Vec::new();
    let mut values: Vec<u32> = Vec::new();

    let mut numbers = input.split(':').nth(1).unwrap().split('|');
    let values_str: String = numbers.clone().nth(1).unwrap().to_string();

    let winning_str: String = numbers.nth(0).unwrap().to_string();

    for num in winning_str.split(' ') {
        if !num.is_empty() {
            winning.push(parse_num(&num.to_string()));
        }
    }

    for num in values_str.split(' ') {
        if !num.is_empty() {
            values.push(parse_num(&num.to_string()));
        }
    }

    (winning, values)
}

/// Count how many winning numbers are in values.
fn calculate_result(winning: &Vec<u32>, values: &Vec<u32>) -> u32 {
    let mut winning_values_count = 0;

    for i in 0..values.len() {
        if winning.contains(&values[i]) {
            winning_values_count += 1;
        }
    }

    winning_values_count
}

/// Calculate the score of the card.
fn calculate_pow(winning: &Vec<u32>, values: &Vec<u32>) -> u32 {
    let winning_values_count = calculate_result(winning, values);

    if winning_values_count <= 2 {
        return winning_values_count;
    }

    2u32.pow(winning_values_count - 1)
}

fn get_lim(i: usize, won_count: u32, len: usize) -> usize {
    min(1 + i + won_count as usize, len)
}

fn main() {
    let input = parse_inputfile(&get_filename());

    let mut winning: Vec<Vec<u32>> = Vec::new();
    let mut numbers: Vec<Vec<u32>> = Vec::new();

    for line in input.iter() {
        let game: (Vec<u32>, Vec<u32>) = get_winning(line);
        winning.push(game.0);
        numbers.push(game.1);
    }
    let mut pow_sum: u32 = 0;
    let mut card_count: u32 = 0;
    let mut cards_won: Vec<u32> = Vec::new();

    cards_won.resize(winning.len(), 1);

    for i in 0..winning.len() {
        pow_sum += calculate_pow(&winning[i], &numbers[i]);
    }

    for i in 0..winning.len() {
        let won_count = calculate_result(&winning[i], &numbers[i]);

        let lim = get_lim(i, won_count, winning.len());

        for j in i + 1..lim {
            cards_won[j] += cards_won[i];
        }
    }

    for i in 0..cards_won.len() {
        card_count += cards_won[i];
    }

    println!("{}, {}", pow_sum, card_count);
}
