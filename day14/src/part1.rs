use core::panic;

use grid::grid;
use grid::Grid;
use itertools::Itertools;

pub(crate) fn part1(file: &String) {
  let mut grid = grid(file);

  print_grid(&grid);

  // move boulders north
  move_boulders(&mut grid);

  println!();
  print_grid(&grid);

  // count weight
  let weight = count_weight(&grid);

  println!("{weight}")
}

fn move_boulders(grid: &mut Grid<char>) {
  // for each column
  // start at the top
  // if empty, add one to 'move_up'
  // if blocked by cube, set move_up to 0
  // if circle and move_up > 0, move circle by nr of move_up
  // and remove 1 from move_up.

  for col in 0..grid.cols() {
    let mut move_up = 0;
    for row in 0..grid.rows() {
      match grid.get(row, col).unwrap() {
        '.' => move_up += 1,
        '#' => move_up = 0,
        'O' => {
          if move_up > 0 {
            *grid.get_mut(row, col).unwrap() = '.';
            *grid.get_mut(row - move_up, col).unwrap() = 'O';
          }
        }
        c => panic!("unexpected char: {c}"),
      }
    }
  }
}

fn count_weight(grid: &Grid<char>) -> usize {
  let height = grid.rows();
  let mut total_weight = 0;
  for i in 0..grid.rows(){
    let weight = height - i;
    total_weight += grid.iter_row(i).filter(|c|**c == 'O').count()* weight
  }
  return total_weight
}

// read each line, if line is empty, create grid
fn grid(file: &String) -> Grid<char> {
  let mut grid = grid![];
  file.lines().for_each(|line| {
    grid.push_row(line.chars().collect_vec());
  });
  grid
}

fn print_grid(grid: &Grid<char>) {
  for row in grid.iter_rows() {
    for c in row {
      print!("{c}");
    }
    println!();
  }
}
