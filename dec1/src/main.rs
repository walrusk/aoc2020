use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let mut winning_numbers: HashMap<i32, Vec<&i32>> = HashMap::new();
  let numbers = read_input("./input.txt");

  // pre-sum pairs
  for n in numbers.iter() {
    for m in numbers.iter() {
      let n_dbl_partner = n + m;
      winning_numbers.insert(n_dbl_partner, vec![n, m]);
    }
  }

  // find third
  for n in numbers.iter() {
    let found = winning_numbers.get(&(2020 - n));
    if !found.is_none() {
      found_win(&n, found.unwrap()[0], found.unwrap()[1]);
      break;
    }
  }
}

fn read_input(filename: &str) -> Vec<i32> {
  let f = File::open(filename).expect("Unable to open file");
  let f = BufReader::new(f);
  return f
    .lines()
    .map(|l| l.unwrap().parse::<i32>().unwrap())
    .collect();
}

fn found_win(num1: &i32, num2: &i32, num3: &i32) {
  println!(
    "we found some winning numbers! {}*{}*{} = {}",
    num1,
    num2,
    num3,
    num1 * num2 * num3
  );
}
