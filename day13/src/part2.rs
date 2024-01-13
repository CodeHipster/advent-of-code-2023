use std::vec;

use grid::grid;
use grid::Grid;
use itertools::Itertools;

pub(crate) fn part2(file: &String) {
  let mut grids = grids(file);
  let mut sum: usize = 0;

  for grid in &mut grids {
    println!();
    // print_grid(grid);
    // println!("columns:");
    sum += column_mirror(grid).iter().sum::<usize>();
    // println!("rows:");
    grid.rotate_left();
    sum += column_mirror(grid).iter().map(|i| i * 100).sum::<usize>();
    println!();
  }

  println!("sum: {sum}")
}

fn column_mirror(grid: &Grid<char>) -> Vec<usize> {
  let mut mirror_column = vec![];
  let width = grid.cols();
  'columns: for ci in 1..grid.cols() {
    // check if they fully mirror with 1 smudge
    // println!("checking column index {ci}");
    let mut left = ci - 1;
    let mut right = ci;
    let mut smudge = 0;
    'mirror: while right < width && smudge <= 1 {
      let l = grid.iter_col(left).cloned().collect::<Vec<char>>();
      let r = grid.iter_col(right).cloned().collect::<Vec<char>>();

      smudge += diff(l, r);

      match left.checked_sub(1) {
        Some(v) => left = v,
        None => break 'mirror,
      };
      right += 1;
    }
    match smudge {
      0 => {
        // println!("part 1 answer: {ci}");
      }
      1 => {
        // println!("part 2 answer: {ci}");
        mirror_column.push(ci);
      }
      _ => continue 'columns,
    }
  }

  return mirror_column;
}

fn diff(left: Vec<char>, right: Vec<char>) -> i32 {
  let mut diff = 0;
  for i in 0..left.len() {
    if left[i] != right[i] {
      // println!("on index {i}: left {:?} not equal to {:?}", left[i], right[i]);
      diff += 1;
    }
  }

  // println!("comparing {left:?}-{right:?} has {diff} differences");
  return diff;
}

// read each line, if line is empty, create grid
fn grids(file: &String) -> Vec<Grid<char>> {
  let mut grids = vec![];
  let mut grid = grid![];
  file.lines().for_each(|line| {
    if line.is_empty() {
      grids.push(grid.clone());
      grid.clear();
    } else {
      grid.push_row(line.chars().collect_vec());
    }
  });
  grids.push(grid);
  grids
}

fn print_grid(grid: &Grid<char>) {
  for row in grid.iter_rows() {
    for c in row {
      print!("{c}");
    }
    println!();
  }
}
