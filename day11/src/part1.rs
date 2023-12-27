use grid::{grid, Grid};

type Location = (usize, usize);

pub(crate) fn part1(file: &String) {
  let mut grid = file.lines().fold(grid![], |mut acc: Grid<char>, line| {
    if !line.chars().any(|c| c != '.') {
      acc.push_row(line.chars().collect());
    }
    acc.push_row(line.chars().collect());
    acc
  });

  add_empty_rows(&mut grid);

  print_grid(&grid);

  let galaxies = map_galaxies(&grid);

  let distance = calc_distances(&galaxies);

  println!("{distance}")
}

fn calc_distances(galaxies: &Vec<Location>) -> i32 {
  let mut total_distance = 0;
  for i in 0..galaxies.len() {
    for j in i + 1..galaxies.len() {
      let y = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs();
      let x = (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs();
      total_distance += x + y;
    }
  }
  return total_distance;
}

fn map_galaxies(grid: &Grid<char>) -> Vec<Location> {
  let mut galaxies = vec![];
  for (ri, row) in grid.iter_rows().enumerate() {
    for (ci, c) in row.enumerate() {
      if c == &'#' {
        galaxies.push((ri, ci))
      }
    }
  }

  return galaxies;
}

fn add_empty_rows(grid: &mut Grid<char>) {
  // TODO don't want to borrow row as mutable.
  let duplicate = grid
    .iter_cols()
    .enumerate()
    .filter_map(|(index, mut col)| {
      if col.by_ref().any(|c| c != &'.') {
        return None;
      }
      return Some(index);
    })
    .collect::<Vec<_>>();

  let height = grid.rows();
  for index in duplicate.into_iter().rev() {
    grid.insert_col(index, vec!['.'; height])
  }
}

fn print_grid(grid: &Grid<char>) {
  for row in grid.iter_rows() {
    for c in row {
      print!("{c}");
    }
    println!();
  }
}
