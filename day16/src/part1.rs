use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use grid::grid;
use grid::Grid;
use itertools::Itertools;

pub(crate) fn part1(file: &String) {
  let contraption = contraption(file);

  // approach as a state problem
  // state is beam direction + location
  // solve until no more states exist.

  // next state is applying beam direction to contraption location
  // mirror = split or passthrough
  // empty = passthrough
  // out of bounds is no new state.

  // store each new state in a set
  // if state already exists in set, don't add it to the stack.

  let initial_state = State {
    row: 0,
    col: 0,
    direction: Direction::RIGHT,
  };
  let mut states = VecDeque::<State>::new();
  let mut visited = HashSet::<State>::new();
  states.push_front(initial_state);

  while let Some(state) = states.pop_back() {
    let next = resolve(&state, &contraption);
    next.into_iter().for_each(|s|{
      if !visited.contains(&s){
        states.push_back(s);
      }
    });
    visited.insert(state);
  }

  let mut v = Grid::new(contraption.rows(), contraption.cols());
  v.fill('.');
  visited.iter().for_each(|state|{
    *v.get_mut(state.row, state.col).unwrap() = '#';
  });

  print_grid(&v);

  let answer = visited.iter().fold(HashSet::new(), |mut acc, s|{
    acc.insert((s.row, s.col));
    acc
  }).len();
  println!("{answer}");
}

fn print_grid(grid: &Grid<char>) {
  for row in grid.iter_rows() {
    for c in row {
      print!("{c}");
    }
    println!();
  }
}

fn resolve(state: &State, contraption: &Grid<char>) -> Vec<State> {
  let c = contraption.get(state.row, state.col).unwrap();

  let mut result = vec![];
  match (c, state.direction) {
    ('.', dir) => {
      // pass right through
      if let Some(s) = next(state.row, state.col, dir, contraption) {
        println!("State:[{state:?}] moved through empty space to: [{s:?}]");
        result.push(s);
      }
    },
    ('|', Direction::LEFT|Direction::RIGHT) =>{
      // split up and down
      if let Some(s) = next(state.row, state.col, Direction::UP, contraption) {
        println!("State:[{state:?}] split on | to: [{s:?}]");
        result.push(s);
      }
      if let Some(s) = next(state.row, state.col, Direction::DOWN, contraption) {
        println!("State:[{state:?}] split on | to: [{s:?}]");
        result.push(s);
      }
    },
    ('|', Direction::UP|Direction::DOWN) =>{
      // pass through
      if let Some(s) = next(state.row, state.col, state.direction, contraption) {
        println!("State:[{state:?}] moved through | to: [{s:?}]");
        result.push(s);
      }
    },
    ('-', Direction::UP|Direction::DOWN) =>{
      // split left and right
      if let Some(s) = next(state.row, state.col, Direction::LEFT, contraption) {
        println!("State:[{state:?}] split on - to: [{s:?}]");
        result.push(s);
      }
      if let Some(s) = next(state.row, state.col, Direction::RIGHT, contraption) {
        println!("State:[{state:?}] split on - to: [{s:?}]");
        result.push(s);
      }
    },
    ('-', Direction::LEFT|Direction::RIGHT) =>{
      // pass through
      if let Some(s) = next(state.row, state.col, state.direction, contraption) {
        println!("State:[{state:?}] moved through - to: [{s:?}]");
        result.push(s);
      }
    },
    ('/', Direction::UP) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::RIGHT, contraption) {
        println!("State:[{state:?}] reflected on / to: [{s:?}]");
        result.push(s);
      }
    },
    ('/', Direction::DOWN) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::LEFT, contraption) {
        println!("State:[{state:?}] reflected on / to: [{s:?}]");
        result.push(s);
      }
    },
    ('/', Direction::RIGHT) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::UP, contraption) {
        println!("State:[{state:?}] reflected on / to: [{s:?}]");
        result.push(s);
      }
    },
    ('/', Direction::LEFT) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::DOWN, contraption) {
        println!("State:[{state:?}] reflected on \\ to: [{s:?}]");
        result.push(s);
      }
    },
    ('\\', Direction::UP) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::LEFT, contraption) {
        println!("State:[{state:?}] reflected on \\ to: [{s:?}]");
        result.push(s);
      }
    },
    ('\\', Direction::DOWN) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::RIGHT, contraption) {
        println!("State:[{state:?}] reflected on \\ to: [{s:?}]");
        result.push(s);
      }
    },
    ('\\', Direction::RIGHT) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::DOWN, contraption) {
        println!("State:[{state:?}] reflected on \\ to: [{s:?}]");
        result.push(s);
      }
    },
    ('\\', Direction::LEFT) =>{
      // reflect up
      if let Some(s) = next(state.row, state.col, Direction::UP, contraption) {
        println!("State:[{state:?}] reflected on / to: [{s:?}]");
        result.push(s);
      }
    }
    _ => {}
  }
  result
}

fn next(row: usize, col: usize, dir: Direction, contraption: &Grid<char>) -> Option<State> {
  match dir {
    Direction::UP => {
      if row > 0 {
        Some(State {
          row: row - 1,
          col,
          direction: dir,
        })
      } else {
        None
      }
    }
    Direction::DOWN => {
      if row + 1 < contraption.rows() {
        Some(State {
          row: row + 1,
          col,
          direction: dir,
        })
      } else {
        None
      }
    }
    Direction::LEFT => {
      if col > 0 {
        Some(State {
          row,
          col: col - 1,
          direction: dir,
        })
      } else {
        None
      }
    }
    Direction::RIGHT => {
      if col + 1 < contraption.cols() {
        Some(State {
          row,
          col: col + 1,
          direction: dir,
        })
      } else {
        None
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct State {
  row: usize,
  col: usize,
  direction: Direction,
}

fn contraption(file: &String) -> Grid<char> {
  let mut grid = grid![];
  file.lines().for_each(|line| {
    grid.push_row(line.chars().collect_vec());
  });
  grid
}
