use std::error::Error;
use std::fs;

const INPUT_FILE: &str = "data/01.txt";
const TARGET: u32 = 2020;

fn part1(input: &[u32]) -> Result<String, &'static str> {
    for i in input {
        for j in input {
            if (i + j) == TARGET {
                return Ok(format!("({} * {}) = {}", i, j, (i * j)));
            }
        }
    }
    Err("answer not found")
}

fn part2(input: &[u32]) -> Result<String, &'static str> {
    for i in input {
        for j in input {
            for k in input {
                if (i + j + k) == TARGET {
                    return Ok(format!("({} * {} * {}) = {}", i, j, k, (i * j * k)));
                }
            }
        }
    }
    Err("answer not found")
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Result<Vec<_>, _> = fs::read_to_string(INPUT_FILE)?
        .lines()
        .map(|x| x.parse())
        .collect();
    let input = input?;
    println!("Part 1 solution: {:?}", part1(&input));
    println!("Part 2 solution: {:?}", part2(&input));
    Ok(())
}
