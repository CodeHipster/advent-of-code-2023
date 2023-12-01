use once_cell::sync::Lazy;
use std::collections::HashMap;

static OPTIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
  HashMap::from([
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
    ("zero", "0"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("0", "0"),
  ])
});

pub(crate) fn part2(file: String) {
  let sum: i32 = file
    .lines()
    .map(|line| {
      (find_first(line) + &find_last(line))
        .parse::<i32>()
        .unwrap()
    })
    .sum();
  println!("{sum}");
}

fn find_last(line: &str) -> String {
  let (_, value) = OPTIONS
    .iter()
    .filter_map(|(key, value)| match line.rfind(key) {
      None => None,
      Some(i) => Some((i, value)),
    })
    .max_by(|(index1, _), (index2, _)| index1.cmp(index2))
    .unwrap();
  return value.to_string();
}

fn find_first(line: &str) -> String {
  let (_, value) = OPTIONS
    .iter()
    .filter_map(|(key, value)| match line.find(key) {
      None => None,
      Some(i) => Some((i, value)),
    })
    .min_by(|(index1, _), (index2, _)| index1.cmp(index2))
    .unwrap();
  return value.to_string();
}
