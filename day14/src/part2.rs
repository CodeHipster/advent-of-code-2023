use core::panic;

use grid::grid;
use grid::Grid;
use itertools::Itertools;

pub(crate) fn part2(file: &String) {
  let mut grid = grid(file);

  print_grid(&grid);

  // cycle boulders
  let total_cycles = 1000000000;
  let mut cycles:Vec<Grid<char>> = vec![];
  for c in 0..total_cycles{
    cycle(&mut grid);
    let w = count_weight(&grid);
    println!("grid at cycle {c}, weight: {w}");
    print_grid(&grid);
    println!();
    match cycles.iter().enumerate().find(|(_, g)|**g == grid){
      Some((i,_)) => {
        println!("recurring pattern from {} to {} cycles", i, c);
        let cycle_length = c - i;
        let cycle_start = i;
        let cycles_left = total_cycles - (i+1);
        let final_state = (cycles_left % cycle_length) + cycle_start;
        grid = cycles[final_state].clone();
        break;
      },
      None => {},
    }
    cycles.push(grid.clone())
  }

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

fn cycle(grid: &mut Grid<char>){
  // north, west, south, east
  move_boulders(grid); // move north
  grid.rotate_right();
  move_boulders(grid); // move west
  grid.rotate_right();
  move_boulders(grid); // move south
  grid.rotate_right();
  move_boulders(grid); // move east
  grid.rotate_right(); // rotate back to original position
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
