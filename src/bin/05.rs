// https://adventofcode.com/2020/day/5

use std::error::Error;
use std::fs;

const INPUT_FILENAME: &str = "data/05.txt";

enum Half {
    Upper,
    Lower,
}

fn partition(half: Half, rows: &[u16]) -> &[u16] {
    match half {
        Half::Lower => &rows[..(rows.len() / 2)],
        Half::Upper => &rows[rows.len() / 2..],
    }
}

fn seat_id(input: &str) -> Result<u16, &'static str> {
    let mut rows: Vec<u16> = (0..128).collect();
    let mut columns: Vec<u16> = (0..8).collect();
    for c in input.chars() {
        match c {
            'F' => rows = partition(Half::Lower, &rows).to_vec(),
            'B' => rows = partition(Half::Upper, &rows).to_vec(),
            'L' => columns = partition(Half::Lower, &columns).to_vec(),
            'R' => columns = partition(Half::Upper, &columns).to_vec(),
            _ => return Err("unexpected control character"),
        }
    }
    Ok(rows[0] * 8 + columns[0])
}

fn main() -> Result<(), Box<dyn Error>> {
    let seats = fs::read_to_string(INPUT_FILENAME)?
        .lines()
        .map(|l| seat_id(l))
        .collect::<Result<Vec<_>, _>>()?;
    for seat in 0..seats.len() {
        if !seats.contains(&(seat as u16))
            && (seat > 0 && seats.contains(&(seat as u16 - 1)))
            && seats.contains(&(seat as u16 + 1))
        {
            println!("seat ID: {}", seat);
        }
    }
    Ok(())
}
