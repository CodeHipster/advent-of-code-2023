use std::collections::BTreeSet;
use std::ops::Add;
use std::vec;

use grid::grid;
use grid::Grid;
use itertools::Itertools;

pub(crate) fn part1(file: &String) {
  // solve it like a variant of a path finding algorithm

  // have a sorted state stack
  // sort on minimal potential heat at the end tile
  // So add the current accumulated heat and the manhattan distance to the end.
  // pop the top and add the possible future states to the stack.

  // The state consists of a direction, steps in line, row, column
  // next states are forward, left, right,
  // or only left and right if steps in line == 3
  // a step left or right resets the steps in line to 1

  let city_map = city_map(file);

  let mut states: BTreeSet<State> = BTreeSet::new();

  let w = distance_to_end(&city_map, 0, 0);
  let initial_state = State {
    weight: w,
    dir: (0, 1),
    line: 0,
    row: 0,
    col: 0,
    heat_loss: 0,
  };
  states.insert(initial_state);

  let mut min = i32::MAX;
  while let Some(state) = states.pop_first() {
    if state.row == city_map.rows()-1 && state.col == city_map.cols()-1{
      if state.heat_loss < min{
        min = state.heat_loss;
        println!("found new min value: {min}");
      }
      continue;
    }
    // early exit, we won't reach faster than previous.
    if state.heat_loss + distance_to_end(&city_map, state.row, state.col)as i32 >= min{
      continue;
    }
    // println!("resolved state: {state:?}");
    let next = resolve(&city_map, state);

    // println!("to {next:?}");
    // break;
    states.extend(next);
  }

  println!("{min}")
}

fn resolve(map: &Grid<char>, state: State) -> Vec<State> {
  let mut states = vec![];
  // forward state
  if state.line < 3 {
    if let Some(pos) = moove(state.dir, (state.row, state.col), map) {
      // println!("pos when moving straight: {pos:?}");
      let hl = map.get(pos.0, pos.1).unwrap().to_digit(10).unwrap();
      let md = distance_to_end(map, pos.0, pos.1);
      states.push(State {
        weight: state.heat_loss as usize + hl as usize + md,
        heat_loss: state.heat_loss + hl as i32,
        line: state.line + 1,
        row: pos.0,
        col: pos.1,
        dir: state.dir,
      })
    }
  }

  // left
  let left = left(state.dir);
  if let Some(pos) = moove(left, (state.row, state.col), map) {
    // println!("pos when moving left: {pos:?}");
    let hl = map.get(pos.0, pos.1).unwrap().to_digit(10).unwrap();
    let md = distance_to_end(map, pos.0, pos.1);
    states.push(State {
      weight: state.heat_loss as usize + hl as usize + md,
      heat_loss: state.heat_loss + hl as i32,
      line: 1,
      row: pos.0,
      col: pos.1,
      dir: left,
    })
  }

  // right
  let right = right(state.dir);
  if let Some(pos) = moove(right, (state.row, state.col), map) {
    // println!("pos when moving right: {pos:?}");
    let hl = map.get(pos.0, pos.1).unwrap().to_digit(10).unwrap();
    let md = distance_to_end(map, pos.0, pos.1);
    states.push(State {
      weight: state.heat_loss as usize + hl as usize + md,
      heat_loss: state.heat_loss + hl as i32,
      line: 1,
      row: pos.0,
      col: pos.1,
      dir: right,
    })
  }

  states
}

// move in direction on grid, (row,col)
// return none if moving out of the grid.
fn moove(dir: (i8, i8), from: (usize, usize), map: &Grid<char>) -> Option<(usize, usize)> {
  // println!("moving from: {from:?} in direction: {dir:?}");
  if (from.0 as i32 + dir.0 as i32) < 0 {
    return None;
  } else if (from.1 as i32 + dir.1 as i32) < 0 {
    return None;
  } else if (from.0 as i32 + dir.0 as i32) >= map.rows() as i32 {
    return None;
  } else if (from.1 as i32 + dir.1 as i32) >= map.cols() as i32 {
    return None;
  } else {
    return Some((
      (from.0 as i32 + dir.0 as i32) as usize,
      (from.1 as i32 + dir.1 as i32) as usize,
    ));
  }
}

fn left(dir: (i8, i8)) -> (i8, i8) {
  (-dir.1, dir.0)
}

fn right(dir: (i8, i8)) -> (i8, i8) {
  (dir.1, -dir.0)
}

fn distance_to_end(map: &Grid<char>, row: usize, col: usize) -> usize {
  ((map.cols() - 1) - col) + ((map.rows() - 1) - row)
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct State {
  weight: usize,
  heat_loss: i32,
  line: i8,
  row: usize,
  col: usize,
  dir: (i8, i8),
}

fn city_map(file: &String) -> Grid<char> {
  let mut grid = grid![];
  file.lines().for_each(|line| {
    grid.push_row(line.chars().collect_vec());
  });
  grid
}
