#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::error::Error;
use std::fs;

const INPUT_FILENAME: &str = "data/02.txt";

fn parse_line(input: &str) -> std::result::Result<regex::Captures<'_>, &str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)\-(\d+) ([[:lower:]]): ([[:lower:]]+)$").unwrap();
    }
    RE.captures(input).ok_or("no match :(")
}

fn check_password(input: &str) -> Result<bool, Box<dyn Error>> {
    let cap = parse_line(input)?;
    Ok((cap[1].parse()?..=cap[2].parse()?).contains(&cap[4].matches(&cap[3]).count()))
}

fn check_password2(input: &str) -> Result<bool, Box<dyn Error>> {
    let cap = parse_line(input)?;
    let desired_char: char = cap[3].parse()?;
    let position1_match = &cap[4]
        .chars()
        .nth(cap[1].parse::<usize>()? - 1)
        .ok_or("no character at position 1")?
        == &desired_char;
    let position2_match = &cap[4]
        .chars()
        .nth(cap[2].parse::<usize>()? - 1)
        .ok_or("no character at position 2")?
        == &desired_char;
    Ok((position1_match || position2_match) && !(position1_match && position2_match))
}

fn main() -> Result<(), Box<dyn Error>> {
    let valid_passwords: Result<Vec<_>, _> = fs::read_to_string(INPUT_FILENAME)?
        .lines()
        .map(|l| check_password(l))
        .collect();
    let count = valid_passwords?.iter().filter(|x| **x).count();
    println!("Valid passwords (part 1): {}", count);
    let valid_passwords: Result<Vec<_>, _> = fs::read_to_string(INPUT_FILENAME)?
        .lines()
        .map(|l| check_password2(l))
        .collect();
    let count = valid_passwords?.iter().filter(|x| **x).count();
    println!("Valid passwords (part 2): {}", count);
    Ok(())
}
