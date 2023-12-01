use std::fs::{File, self};
use std::io::{self, BufRead};
use std::iter::Sum;
use std::path::Path;

pub(crate) fn part1() {
  let sum: i32 = read_file("./input.txt")
  .lines().map(|line| {
    // Not very efficient.
    let mut calibration = String::from("");
    // find first number
    let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
    calibration.push(line.chars().nth(first).unwrap());
    // find last number
    let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    calibration.push(line.chars().nth(last).unwrap());
    calibration.parse::<i32>().unwrap()
  }).sum();
  
  println!("{sum}");
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}

// fn type_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
