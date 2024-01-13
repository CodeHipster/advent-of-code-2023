use grid::grid;
use grid::Grid;
use itertools::Itertools;

pub(crate) fn part1(file: &String) {
  let mut grids = grids(file);
  let mut sum: i32 = 0;

  println!("iterating over columns");
  for grid in &mut grids {
    println!();
    print_grid(grid);
    println!("columns:");
    sum += column_mirror(grid).iter().map(|i|(i+1)).sum::<i32>();
    println!("rows:");
    grid.rotate_left();
    sum += column_mirror(grid).iter().map(|i|(i+1)*100).sum::<i32>();
    println!();
  }

  println!("sum: {sum}")
}

fn column_mirror(grid: &Grid<char>) -> Vec<i32> {
  return grid
    .iter_cols()
    .tuple_windows()
    .enumerate()
    .filter_map(|(index, (c1, c2))| {
      // find mirroring indexes
      let c1 = c1.collect::<String>();
      let c2 = c2.collect::<String>();
      if c1 == c2 {
        let m = (index) as i32;
        // println!("mirror after index: {m}. {c1}-{c2}");
        return Some(m);
      } else {
        return None;
      }
    })
    .filter(|index| {
      // check if they fully mirror
      let width = grid.cols() as i32;
      let mut left = *index - 1;
      let mut right = *index + 2;
      while left >= 0 && right < width {
        let l = grid.iter_col(left as usize).cloned().collect::<Vec<char>>();
        let r = grid
          .iter_col(right as usize)
          .cloned()
          .collect::<Vec<char>>();
        if l != r {
          return false;
        } else {
          left -= 1;
          right += 1;
        }
      }
      println!("mirror after: {index}");
      return true;
    })
    .collect::<Vec<i32>>();
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
