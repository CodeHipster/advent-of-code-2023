use core::time;
use std::{collections::HashMap, thread, vec, panic::UnwindSafe};

use gcd::Gcd;
use lazy_static::lazy_static;
use num_bigint::BigInt;
use regex::Regex;

pub(crate) fn part2(file: &String) {
  let mut lines = file.lines();
  let directions_string = lines.next().unwrap();
  let mut directions = map_directions(directions_string);
  lines.next();

  let nodes = lines.map(|line| map_node(line)).collect::<Vec<_>>();

  let node_map = nodes
    .iter()
    .map(|node| (node.id.clone(), node))
    .collect::<HashMap<_, _>>();

  // walk through the directions

  let mut step_count = 0;

  let mut indexes = node_map
    .iter()
    .filter(|(index, _)| index.ends_with('A'))
    .map(|(index, _)| index)
    .cloned()
    .collect::<Vec<_>>();

  let mut steps = indexes.iter().map(|seed| {
    directions.reset();
    let iterations = find_loop(seed, &node_map, &mut directions);
    // println!("iterations: {}", iterations);
    return iterations
  }).collect::<Vec<_>>();
  
  
  steps.sort();
  let steps = steps.iter().rev().fold(1u64,|acc, steps| {
    let cast = *steps as u64;
    println!("acc:{acc}, steps:{steps}, mod?: {}", &acc % cast);
    let gcd = Gcd::gcd(acc, cast);
    return (acc * cast) / gcd
  });
  
  println!("{steps:?}");
}

fn find_loop(seed: &str, node_map: &HashMap<String, &Node>, directions: &mut Directions) -> i32{
  // step through node map and record every end node we reach. until we detect a loop.
  // map with as index a tuple of (node id, dir index) and value step count.
  let mut end_map: HashMap<(&str, usize), i32> = HashMap::new();

  let mut index = seed;
  while let Some(step) = directions.next() {
    // println!("Direction: {:?}", direction);
    index = node_map.get(index).unwrap().get(&step.dir);
    // println!("Next index: {index}");
    if index.ends_with('Z') {
      let prev = end_map.insert((index, step.dir_index), step.count);
      if let Some(prev_count) = prev{
        println!("Seed: {seed} reached state: ({}, {}) before on step: {} and now on step: {}", index, step.dir_index, prev_count, step.count);
        println!("{:?}", end_map);
        return step.count - prev_count;
      }
    }
  }
  panic!("There is always a step.")
}

struct Recurrence {
  start: i32,
  repeat_every: i32,
}

pub struct Directions {
  dirs: Vec<Direction>,
  index: usize,
  count: i32,
}

pub struct Step {
  dir: Direction,
  dir_index: usize,
  count: i32,
}

impl Iterator for Directions {
  type Item = Step;

  fn next(&mut self) -> Option<Self::Item> {
    let dir = self.dirs.get(self.index);
    self.index += 1;
    self.count += 1;
    if self.index >= self.dirs.len() {
      self.index = 0
    }
    let step = Step {
      count: self.count,
      dir: dir.unwrap().clone(),
      dir_index: self.index,
    };
    return Some(step);
  }
}

impl Directions {
  pub fn new(dirs: Vec<Direction>) -> Directions {
    Directions {
      dirs,
      index: 0,
      count: 0,
    }
  }

  fn reset(&mut self){
    self.index = 0;
    self.count = 0;
  }
}

#[derive(Clone, Debug)]
pub enum Direction {
  Left,
  Right,
}

pub struct Node {
  pub id: String,
  pub left: String,
  pub right: String,
}

impl Node {
  pub fn get(&self, dir: &Direction) -> &str {
    match dir {
      Direction::Left => &self.left,
      Direction::Right => &self.right,
    }
  }
}

pub fn map_node(line: &str) -> Node {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
  }
  let captures = RE.captures(line).unwrap();
  let id = captures.get(1).unwrap().as_str().to_owned();
  let left = captures.get(2).unwrap().as_str().to_owned();
  let right = captures.get(3).unwrap().as_str().to_owned();
  Node { id, left, right }
}

pub fn map_directions(line: &str) -> Directions {
  let dirs = line
    .chars()
    .map(|c| match c {
      'L' => Direction::Left,
      'R' => Direction::Right,
      _ => panic!("unexpected direction {c}"),
    })
    .collect::<Vec<_>>();
  Directions::new(dirs)
}
