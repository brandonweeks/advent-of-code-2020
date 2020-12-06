use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut anyone_answered, mut all_answered) = (0, 0);
    for group in fs::read_to_string("data/06.txt")?.split("\n\n") {
        let mut people = 0;
        let mut answers = HashMap::new();
        for l in group.lines() {
            for c in l.chars() {
                *answers.entry(c).or_insert(0) += 1;
            }
            people += 1;
        }
        anyone_answered += answers.iter().count();
        all_answered += answers.iter().filter(|&(_, v)| people == *v).count();
    }
    println!("Anyone: {}, All: {}", anyone_answered, all_answered);
    Ok(())
}
