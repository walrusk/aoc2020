// use std::collections::HashMap;
extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let f = File::open("./input.txt").expect("Unable to open file");
  let f = BufReader::new(f);
  let mut num_valid_passwords = 0;

  for line in f.lines() {
    let l = line.expect("Unable to read line");
    let (position_a, position_b, letter, password) = match_inane_pw_requirements(&l);
    let mut matches = 0;
    if letter_at_position(password, position_a) == letter {
      matches+=1;
    }
    if letter_at_position(password, position_b) == letter {
      matches+=1;
    }
    if matches == 1 {
      num_valid_passwords+=1;
    }
  }

  println!("Aaaaaaaaand the number of valid passwords is: {}", num_valid_passwords);
}

fn letter_at_position(s: &str, pos: usize) -> String {
  let l = s.chars().nth(pos).unwrap().to_string();
  return String::from(l);
}

fn match_inane_pw_requirements(pw_line: &str) -> (usize, usize, &str, &str) {
  let rg_w_named = Regex::new(r"(?P<position_a>\d+)-(?P<position_b>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
  return match rg_w_named.captures(pw_line) {
    Some(x) => (
      x.name("position_a").unwrap().as_str().parse::<usize>().unwrap() - 1,
      x.name("position_b").unwrap().as_str().parse::<usize>().unwrap() - 1,
      x.name("letter").unwrap().as_str(),
      x.name("password").unwrap().as_str(),
    ),
    None => unreachable!(),
  };
}
