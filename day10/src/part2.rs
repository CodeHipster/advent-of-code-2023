use std::{fmt::Display, mem::swap, ops::Add};
use std::collections::HashSet;  

use grid::{grid, Grid};

type Location = (usize, usize, char);

pub(crate) fn part2(file: &String) {
  let grid = file
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .fold(grid![], |mut acc: Grid<char>, chars| {
      acc.push_row(chars);
      acc
    });

  let start = find_start(&grid);
  let mut pipe = get_loop(&grid, &start);
  // set start shape of pipe.
  let start_shape = start_pipe_shape(&pipe);
  pipe[0].2 = start_shape;

  // transform pipe into a hashmap
  // transform S to the pipe element it resembles

  // scan each line
  // when reaching a pipe element mark up and/or down flag
  // when reaching a non pipe element count as inner if both up and down are flagged
  // | flags up and down
  // - flags none
  // L & J flags up
  // 7 & F flag down


  println!("{pipe:?}")
}

fn start_pipe_shape(pipe: &Vec<Location>) -> char {
  // get dir of pipe elements around start.
  let start = pipe[0];
  let second = pipe[1];
  let last = pipe.last().unwrap();

  let from = get_offset(&start,last);
  let to = get_offset(&second, &start);

  let from_options = match from {
    (0, 1) => HashSet::from(['7', 'J', '-']),  // from left
    (0, -1) => HashSet::from(['F', 'L', '-']), // from right
    (1, 0) => HashSet::from(['L', 'J', '|']),  // from top
    (-1, 0) => HashSet::from(['F', '7', '|']), // from bottom
    _ => panic!("Couldn't find a fitting pipe shape for start."),
  };

  let to_options = match to {
    (0, 1) => HashSet::from(['F', 'L', '-']),  // to right
    (0, -1) => HashSet::from(['7', 'J', '-']), // to left
    (1, 0) => HashSet::from(['F', '7', '|']),  // to bottom
    (-1, 0) => HashSet::from(['L', 'J', '|']), // to top
    _ => panic!("Couldn't find a fitting pipe shape for start."),
  };
  // find the shape that is in the from & to list.

  let mut intersection = from_options.intersection(&to_options); 
  let start_shape = *(intersection.next().unwrap());

  println!("from: {}, start: {start_shape}, to: {}", last.2, second.2);
  return start_shape;
}

fn get_offset(from: &Location, to: &Location) -> (i32, i32) {
  (from.0 as i32 - to.0 as i32, from.1 as i32 - to.1 as i32)
}

fn get_loop(grid: &Grid<char>, start: &Location) -> Vec<Location> {
  let starting_pipes = find_connecting_pipes(&start, &grid);

  for pipe in &starting_pipes {
    if let Some(steps) = steps_to_start(pipe, &grid, &start) {
      return steps;
    }
  }
  panic!("Didn't find any loop")
}

fn steps_to_start(pipe: &Location, grid: &Grid<char>, start: &Location) -> Option<Vec<Location>> {
  let mut steps: Vec<Location> = vec![];
  steps.push(*start);
  steps.push(*pipe);
  let mut next = *pipe;
  let mut prev = *start;
  loop {
    let connecting = find_connecting_pipes(&next, grid)
      .into_iter()
      .filter(|n| !(n.0 == prev.0 && n.1 == prev.1 && n.2 == prev.2))
      .collect::<Vec<_>>();
    if connecting.is_empty() {
      // println!("end of pipe.");
      return None;
    }
    if connecting.len() > 1 {
      // print!("pipe goes into multiple directions.");
      return None;
    }
    if connecting[0].2 == 'S' {
      // println!("reached start again!");
      return Some(steps);
    }

    // println!("{connecting:?}");
    prev = next;
    next = connecting[0];
    steps.push(next);
  }
}

fn find_connecting_pipes(&(row, col, pipe): &Location, grid: &Grid<char>) -> Vec<Location> {
  let offset_list: Vec<(&str, i8, i8)> = vec![
    ("up", -1, 0),
    ("right", 0, 1),
    ("down", 1, 0),
    ("left", 0, -1),
  ];

  offset_list
    .iter()
    .filter(|(dir, _, _)| {
      match (dir, pipe) {
        // Filter out directions the current pipe can't go
        (&"up", '|' | 'J' | 'L' | 'S') => true,
        (&"right", '-' | 'F' | 'L' | 'S') => true,
        (&"down", '|' | 'F' | '7' | 'S') => true,
        (&"left", '-' | 'J' | '7' | 'S') => true,
        _ => {
          // println!("current pipe: {pipe} can't go to {dir}");
          false
        }
      }
    })
    .filter_map(|(dir, ro, co)| {
      // Filter out locations that are outside the grid and map to neighbor location
      let row_add = add_bounded(row, *ro, grid.rows());
      let col_add = add_bounded(col, *co, grid.cols());
      match (row_add, col_add) {
        (Some(r), Some(c)) => Some((dir, r, c)),
        _ => {
          // println!("{row} {ro} {col} {co} row add:{row_add:?}, col add{col_add:?}");
          None
        }
      }
    })
    .filter_map(|(dir, row, col)| {
      // filter out neighbor pipes that can't connect with the current pipe.
      let c = grid.get(row, col).unwrap();
      match (dir, c) {
        (&"up", '|' | 'F' | '7' | 'S') => Some((row, col, *c)),
        (&"right", '-' | 'J' | '7' | 'S') => Some((row, col, *c)),
        (&"down", '|' | 'J' | 'L' | 'S') => Some((row, col, *c)),
        (&"left", '-' | 'F' | 'L' | 'S') => Some((row, col, *c)),
        _ => {
          // println!("can't reach neighbor pipe: {c} from {dir}");
          None
        }
      }
    })
    .collect::<Vec<_>>()
}

fn add_bounded(lhs: usize, rhs: i8, max: usize) -> Option<usize> {
  if rhs.is_negative() {
    let (lhs, overflow) = lhs.overflowing_sub(rhs.abs() as usize);
    if overflow {
      return None;
    }
    return Some(lhs);
  } else {
    let add = lhs.add(rhs as usize);
    if add >= max {
      return None;
    }
    return Some(add);
  }
}

fn find_start(grid: &Grid<char>) -> Location {
  grid
    .iter_rows()
    .enumerate()
    .find_map(|(ri, row)| {
      let opt_ci = row
        .enumerate()
        .find_map(|(ci, col)| if *col == 'S' { Some(ci) } else { None });
      if let Some(ci) = opt_ci {
        Some((ri, ci, 'S'))
      } else {
        None
      }
    })
    .unwrap()
}
