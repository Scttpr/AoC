use fancy_regex::{Regex, Error}; 
use crate::parser::{read_input, InputType};

pub fn run () {
    println!("DAY 1");

    let input = read_input(1, &InputType::Full);
    let first_total = part_one(&input);
    let second_total = part_two(&input).unwrap();

    println!("part A calibration total is {first_total}");
    println!("part B calibration total is {second_total}");
    println!("\n");
}

fn part_one (input: &[String]) -> u32  {
    input
        .iter()
        .map(|line| line.chars()
            .filter(|chr| chr.is_numeric())
            .collect::<Vec<char>>()
        )
        .filter_map(get_calibration)
        .sum()
}

fn part_two (input: &[String]) -> Result<u32, Error> {
    let re = Regex::new(r"(?<=o)ne|(?<=t)wo|(?<=t)hree|four|five|six|seven|(?<=e)ight|(?<=n)ine|[1-9]")?;

    Ok(input.
        iter()
        .map(|line| {
            re.find_iter(line)
                .filter_map(|m| get_number(m.unwrap().as_str()))
                .collect::<Vec<char>>()
        })
        .filter_map(get_calibration)
        .sum())
}

fn get_number(str: &str) -> Option<char> {
    match str {
        "1" | "ne" => Some('1'),
        "2" | "wo" => Some('2'),
        "3" | "hree" => Some('3'),
        "4" | "four" => Some('4'),
        "5" | "five" => Some('5'),
        "6" | "six" => Some('6'),
        "7" | "seven" => Some('7'),
        "8" | "ight" => Some('8'),
        "9" | "ine" => Some('9'),
        _ => None
    }
}

fn get_calibration(line: Vec<char>) -> Option<u32> {
    let first = line.first()?;
    let last = line.last()?;
    
    format!("{first}{last}")
        .parse()
        .ok()
}