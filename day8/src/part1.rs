use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub(crate) fn part1(file: &String) {
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
  let mut index = "AAA";
  while let Some(direction) = directions.next() {
    // println!("Direction: {:?}", direction);
    step_count += 1;
    index = node_map.get(index).unwrap().get(&direction);
    // println!("Next index: {index}");
    if index == "ZZZ" {
      break;
    }
  }

  println!("{step_count}");
}


pub struct Directions {
  dirs: Vec<Direction>,
  index: usize,
}

impl Iterator for Directions {
  type Item = Direction;

  fn next(&mut self) -> Option<Self::Item> {
    let dir = self.dirs.get(self.index);
    self.index += 1;
    if self.index >= self.dirs.len() {
      self.index = 0
    }
    return Some(dir.unwrap().clone());
  }
}

impl Directions {
  pub fn new(dirs: Vec<Direction>) -> Directions {
    Directions { dirs, index: 0 }
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