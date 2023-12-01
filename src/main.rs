#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod parser;
mod day1;

fn main() {
    println!("Hello world! Here is Advent of code 2023\n");
    
    day1::run();
}
