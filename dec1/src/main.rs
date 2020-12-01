use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let f = File::open("./input.txt").expect("Unable to open file");
  let f = BufReader::new(f);
  let mut winning_numbers = HashMap::new();

  for line in f.lines() {
    let n = line.expect("Unable to read line");
    let n = n.parse::<i32>().unwrap();
    let n_partner = 2020 - n;
    let found = winning_numbers.get(&n);
    if !found.is_none() {
      found_win(n, n_partner);
      break;
    }
    winning_numbers.insert(n_partner, 1);
  }
}

fn found_win(num1: i32, num2: i32) {
  println!(
    "we found a winning number! {}*{} = {}",
    num1,
    num2,
    num1 * num2
  );
}
