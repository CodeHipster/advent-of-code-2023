use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

use grid::grid;
use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
  x: i32,
  y: i32,
}

impl Add for Coord {
  type Output = Coord;

  fn add(self, rhs: Self) -> Self::Output {
    let x = self.x + rhs.x;
    let y = self.y + rhs.y;
    Coord { x, y }
  }
}

impl Into<(i32, i32)> for Coord {
  fn into(self) -> (i32, i32) {
    (self.x, self.y)
  }
}

impl From<(i32, i32)> for Coord {
  fn from(c: (i32, i32)) -> Coord {
    Coord { x: c.0, y: c.1 }
  }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
  LEFT,
  RIGHT,
  UP,
  DOWN,
}

impl Direction{
  fn turn_left(&self) -> Direction{
    match self {
        Direction::LEFT => Self::DOWN,
        Direction::RIGHT => Self::UP,
        Direction::UP => Self::LEFT,
        Direction::DOWN => Self::RIGHT,
    }
  }

  fn turn_right(&self) -> Direction{
    match self {
        Direction::LEFT => Self::UP,
        Direction::RIGHT => Self::DOWN,
        Direction::UP => Self::RIGHT,
        Direction::DOWN => Self::LEFT,
    }
  }
}

impl TryFrom<Coord> for Direction {
  type Error = String;
  fn try_from(dir: Coord) -> Result<Self, Self::Error> {
    match dir {
      Coord { x: 1, y: 0 } => Ok(Direction::RIGHT),
      Coord { x: -1, y: 0 } => Ok(Direction::LEFT),
      Coord { x: 0, y: -1 } => Ok(Direction::UP),
      Coord { x: 0, y: 1 } => Ok(Direction::DOWN),
      _ => Err(format!("Can't form a direction from {dir:?}")),
    }
  }
}

impl Into<(i32, i32)> for Direction {
  fn into(self) -> (i32, i32) {
    match self {
      Direction::RIGHT => (1, 0),
      Direction::LEFT => (-1, 0),
      Direction::UP => (0, -1),
      Direction::DOWN => (0, 1),
    }
  }
}

impl Into<Coord> for Direction {
  fn into(self) -> Coord {
    match self {
      Direction::RIGHT => (1, 0).into(),
      Direction::LEFT => (-1, 0).into(),
      Direction::UP => (0, -1).into(),
      Direction::DOWN => (0, 1).into(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Crucible {
  heat_loss: u32,
  pos: Coord,
  dir: Direction,
  in_line: u8,
}


impl Ord for Crucible {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      other.heat_loss.cmp(&self.heat_loss)
  }
}

impl PartialOrd for Crucible {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
  }
}

pub(crate) fn part2(file: &String) {
  let city_map = city_map(file);

  let mut states: BinaryHeap<Crucible> = BinaryHeap::new();
  let mut seen = HashSet::new();

  let initial_state_right = Crucible {
    heat_loss: 0,
    pos: (0, 0).into(),
    dir: Direction::RIGHT,
    in_line: 0,
  };

  let initial_state_down = Crucible {
    heat_loss: 0,
    pos: (0, 0).into(),
    dir: Direction::DOWN,
    in_line: 0,
  };
  
  states.push(initial_state_down);
  states.push(initial_state_right);

  println!("{states:?}");
  let end_pos = Coord {
    x: (city_map.cols() - 1) as i32,
    y: (city_map.rows() - 1) as i32,
  };
  let mut min = u32::MAX;
  while let Some(crucible) = states.pop() {
    if end_pos == crucible.pos && crucible.in_line >= 4 {
      if crucible.heat_loss < min {
        min = crucible.heat_loss;
        println!("found new min value: {min}");
      }
      continue;
    }

    // println!("resolving state: {crucible:?}");
    let next = resolve(&city_map, crucible);
    // println!("to {next:?}");

    for crucible in next{
      if seen.insert((crucible.pos, crucible.dir, crucible.in_line)){
        states.push(crucible);
      }
    }

    // sleep(Duration::from_millis(100));
  }

  println!("{min}")
}

fn resolve(map: &Grid<u32>, crucible: Crucible) -> Vec<Crucible> {
  let mut states = vec![];
  // forward state
  if crucible.in_line < 9 {
    if let Some(pos) = moove(crucible.dir, crucible.pos, map) {
      // println!("pos when moving straight: {pos:?}");
      let hl = map.get(pos.y, pos.x).unwrap();
      states.push(Crucible {
        heat_loss: crucible.heat_loss + hl,
        in_line: crucible.in_line + 1,
        pos,
        dir: crucible.dir,
      })
    }
  }

  if crucible.in_line >= 4 {
    // left
    let left = crucible.dir.turn_left();
    if let Some(pos) = moove(left, crucible.pos, map) {
      // println!("pos when moving left: {pos:?}");
      let hl = map.get(pos.y, pos.x).unwrap();
      states.push(Crucible {
        heat_loss: crucible.heat_loss + hl,
        in_line: 1,
        pos,
        dir: left,
      })
    }

    // right
    let right = crucible.dir.turn_right();
    if let Some(pos) = moove(right, crucible.pos, map) {
      // println!("pos when moving right: {pos:?}");
      let hl = map.get(pos.y, pos.x).unwrap();
      states.push(Crucible {
        heat_loss: crucible.heat_loss + hl,
        in_line: 1,
        pos,
        dir: right,
      })
    }
  }

  states
}

// move in direction on grid, (row,col)
// return none if moving out of the grid.
fn moove(dir: Direction, from: Coord, map: &Grid<u32>) -> Option<Coord> {
  let dir: Coord = dir.into();
  // println!("moving from: {from:?} in direction: {dir:?}");
  let result = from + dir;
  if result.x < 0 || result.y < 0 || result.x >= map.cols() as i32 || result.y >= map.rows() as i32
  {
    return None;
  } else {
    return Some(from + dir);
  }
}

fn city_map(file: &String) -> Grid<u32> {
  let mut grid = grid![];
  file.lines().for_each(|line| {
    grid.push_row(line.chars().map(|c|c.to_digit(10).unwrap()).collect_vec());
  });
  grid
}
