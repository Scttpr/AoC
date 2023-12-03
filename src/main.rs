#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod parser;
mod day1;
mod day2;
mod day3;

fn main() {
    println!("Hello world! Here is Advent of code 2023\n");
    
    day1::run();
    day2::run();
    day3::run();
}