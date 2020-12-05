#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::error::Error;
use std::fs;

const INPUT_FILENAME: &str = "data/04.txt";

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
}

impl Passport {
    fn new(input: &str) -> Passport {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([[:lower:]]{3,}):(\S+)").unwrap();
        }
        let mut passport: Passport = Default::default();
        for cap in RE.captures_iter(input) {
            match &cap[1] {
                "byr" => passport.birth_year = Some(cap[2].parse().unwrap()),
                "iyr" => passport.issue_year = Some(cap[2].parse().unwrap()),
                "eyr" => passport.expiration_year = Some(cap[2].parse().unwrap()),
                "hgt" => passport.height = Some(cap[2].to_string()),
                "hcl" => passport.hair_color = Some(cap[2].to_string()),
                "ecl" => passport.eye_color = Some(cap[2].to_string()),
                "pid" => passport.passport_id = Some(cap[2].to_string()),
                "cid" => (),
                _ => println!("unknown key: {}", &cap[1]), // TODO
            }
        }
        passport
    }

    fn validate(&self) -> bool {
        if self.birth_year.is_none() {
            return false;
        }
        if self.issue_year.is_none() {
            return false;
        }
        if self.expiration_year.is_none() {
            return false;
        }
        if self.height.is_none() {
            return false;
        }
        if self.hair_color.is_none() {
            return false;
        }
        if self.eye_color.is_none() {
            return false;
        }
        if self.passport_id.is_none() {
            return false;
        }
        true
    }

    fn validate2(&self) -> bool {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
            static ref HAIR_COLOR_RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
            static ref EYE_COLOR_RE: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
        }
        if self.birth_year.is_none() || !(1920..=2002).contains(&self.birth_year.unwrap()) {
            return false;
        }
        if self.issue_year.is_none() || !(2010..=2020).contains(&self.issue_year.unwrap()) {
            return false;
        }
        if self.expiration_year.is_none() || !(2020..=2030).contains(&self.expiration_year.unwrap()) {
            return false;
        }
        if self.height.is_none() {
            return false;
        }
        let height_match = HEIGHT_RE.captures(&self.height.as_ref().unwrap());
        if height_match.is_none() {
            return false;
        }
        let height_caps = height_match.unwrap();
        match &height_caps[2] {
            "cm" => {
                if !(150..=193).contains(&height_caps[1].parse().unwrap()) {
                    return false;
                }
            },
            "in" => {
                if !(59..=76).contains(&height_caps[1].parse().unwrap()) {
                    return false;
                }
            },
            _ => println!("bad!"),
        }
        if self.hair_color.is_none() {
            return false;
        }
        if HAIR_COLOR_RE.captures(&self.hair_color.as_ref().unwrap()).is_none() {
            return false;
        }
        if self.eye_color.is_none() {
            return false;
        }
        if EYE_COLOR_RE.captures(&self.eye_color.as_ref().unwrap()).is_none() {
            return false;
        }
        if self.passport_id.is_none() {
            return false;
        }
        if self.passport_id.as_ref().unwrap().len() != 9 {
            return false;
        }
        true
    }
}

struct Reader<'a> {
    input: std::iter::Peekable<std::str::Lines<'a>>,
}

impl Reader<'_> {
    fn new(input: &str) -> Reader {
        Reader {
            input: input.lines().peekable(),
        }
    }
}

impl Iterator for Reader<'_> {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = Vec::new();
        loop {
            let l = self.input.next()?;
            vec.push(l);
            if l.is_empty() || self.input.peek().is_none() {
                let s = vec.join("\n");
                return Some(Passport::new(&s));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let foo = fs::read_to_string(INPUT_FILENAME)?;
    // Part 1
    let r = Reader::new(&foo);
    let valid = r.map(|p| p.validate()).filter(|p| *p).count();
    println!("valid: {}", valid);
    // Part 2
    let r = Reader::new(&foo);
    let valid = r.map(|p| p.validate2()).filter(|p| *p).count();
    println!("valid: {}", valid);
    Ok(())
}
