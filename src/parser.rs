use std::{fs, env};

pub enum InputType {
    Sample,
    Full 
}

pub fn read_input(day: u8, input_type: &InputType) -> Vec<String> {
    let dir = env::current_dir()
        .unwrap()
        .display()
        .to_string();
    let filename = match input_type {
        InputType::Sample => format!("day{day}/sample.txt"),
        InputType::Full => format!("day{day}/input.txt")
    };
    let path = format!("{dir}/src/{filename}");

    fs::read_to_string(path)
        .expect("file not found")
        .lines()
        .map(String::from)
        .collect()
}