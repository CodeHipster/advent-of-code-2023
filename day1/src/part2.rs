use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::iter::Sum;
use std::path::Path;

static OPTIONS: Lazy<HashMap<&str, i32>> = Lazy::new(|| HashMap::from([
  ("one", 1),
  ("two", 2),
  ("three", 3),
  ("four", 4),
  ("five", 5),
  ("six", 6),
  ("seven", 7),
  ("eight", 8),
  ("nine", 9),
  ("zero", 0),
  ("1", 1),
  ("2", 2),
  ("3", 3),
  ("4", 4),
  ("5", 5),
  ("6", 6),
  ("7", 7),
  ("8", 8),
  ("9", 9),
  ("0", 0),
]));

pub(crate) fn part2(file: String) {
    let sum: i32 = file
        .lines()
        .map(|line| {
            // Not very efficient.
            let mut calibration = String::from("");
            // find first number
            findFirst(line)
            // find last number
            // let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            // calibration.push(line.chars().nth(last).unwrap());
            // calibration.parse::<i32>().unwrap()
        })
        .sum();

    println!("{sum}");
}

fn findFirst(line: &str) -> i32 {
    let (index, value) = OPTIONS
        .iter()
        .filter_map(|(key, value)| {
          match line.find(key){
            None => None,
            Some(i) => Some((i, value))
          }
        })
        .min_by(|(index1, _), (index2, _)| index1.cmp(index2))
        .unwrap();

    println!("{index}, {value}");
    return *value;
}

// fn type_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
