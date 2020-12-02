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
    let (min, max, letter, password) = match_inane_pw_requirements(&l);
    let num_letters = password.matches(letter).count() as i32;
    if num_letters >= min && num_letters <= max {
      num_valid_passwords += 1;
    }
  }

  println!("Aaaaaaaaand the number of valid passwords is: {}", num_valid_passwords);
}

fn match_inane_pw_requirements(pw_line: &str) -> (i32, i32, &str, &str) {
  let rg_w_named = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
  return match rg_w_named.captures(pw_line) {
    Some(x) => (
      x.name("min").unwrap().as_str().parse::<i32>().unwrap(),
      x.name("max").unwrap().as_str().parse::<i32>().unwrap(),
      x.name("letter").unwrap().as_str(),
      x.name("password").unwrap().as_str(),
    ),
    None => unreachable!(),
  };
}
