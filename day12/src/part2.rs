use cached::proc_macro::cached;
use std::collections::VecDeque;

pub(crate) fn part2(file: &String) {
  let possibilities: i64 = file
    .lines()
    .map(|line| {
      let split = line.split(" ").collect::<Vec<_>>();
      let pattern = split[0].to_owned() + "?" + split[0] + "?" + split[0] + "?" + split[0] + "?" + split[0] + ".";
      
      let groups_str = split[1].to_owned() + "," + split[1] + "," + split[1] + "," + split[1] + "," + split[1];
      let groups = groups_str
        .split(',')
        .map(|group| group.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

      // println!("Calculating possibilities for pattern: {pattern}");
      let p = possibilities(pattern.to_owned(), groups);
      // println!(
      //   "{p} possibilities for pattern: {pattern}, with groups: {}",
      //   split[1]
      // );
      p
    })
    .sum();

  println!("{possibilities}");
}

#[cached]
fn possibilities(pattern: String, mut groups: VecDeque<usize>) -> i64 {
  if pattern.is_empty() {
    return 1;
  }
  let group = groups.pop_front();
  // println!("removed group: {group:?}");

  let max_len = pattern.len() - groups.iter().fold(0, |acc, g| acc + 1 + g);
  // println!("length of pattern to generate subpatterns for: {max_len}");

  let sub_patterns = patterns(pattern[..max_len].to_owned(), group);
  // println!("Valid subpatterns: {sub_patterns:?}");

  let possibilities = sub_patterns.into_iter().fold(0, |acc, sub_pattern| {
    let p = pattern[sub_pattern.len()..].to_owned();
    acc + possibilities(p, groups.clone()) // Recursion...
  });
  // println!("{possibilities} possibilities for {pattern} and groups: {group:?}-{groups:?}");
  return possibilities;
}

#[cached]
fn patterns(pattern: String, group: Option<usize>) -> Vec<String> {
  // If there are no more groups, all of the remaining pattern must be '.'
  if group.is_none() {
    let p: String = vec!['.'; pattern.len()].iter().collect();
    if validate(p.clone(), pattern.clone()) {
      return vec![p];
    } else {
      return vec![];
    }
  }

  let group = group.unwrap();

  let mut patterns = vec![];
  let mut start = 0;
  while start + group < pattern.len() {
    // <, not checking == as the final char is always '.', to separate groups
    let base = vec!['.'; start + group];
    let mut generated = base.clone();
    for i in 0..group {
      generated[i + start] = '#';
    }
    let generated = generated.iter().collect::<String>() + ".";
    if validate(generated.clone(), pattern.clone()) {
      patterns.push(generated);
    }
    start += 1;
  }
  return patterns;
}

#[cached]
fn validate(generated: String, pattern: String) -> bool {
  let pat_char = pattern.chars().collect::<Vec<_>>();
  let gen_char = generated.chars().collect::<Vec<_>>();

  // Result could be shorter than pattern, only check if the result so far is correct.
  for i in 0..gen_char.len() {
    match pat_char[i] {
      '?' => {
        continue;
      }
      c => {
        if c != gen_char[i] {
          return false;
        }
      }
    }
  }
  return true;
}
