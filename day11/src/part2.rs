use grid::{grid, Grid};

type Location = (usize, usize);

pub(crate) fn part2(file: &String) {
  let mut grid = file.lines().fold(grid![], |mut acc: Grid<char>, line| {
    acc.push_row(line.chars().collect());
    acc
  });

  let empty_rows = get_empty_rows(&mut grid);
  let empty_colums = get_empty_colums(&mut grid);

  print_grid(&grid);
  println!("empty rows: {empty_rows:?}");
  println!("empty cols: {empty_colums:?}");

  let galaxies = map_galaxies(&grid, empty_rows, empty_colums);

  let distance = calc_distances(&galaxies);

  println!("{distance}")
}

fn calc_distances(galaxies: &Vec<Location>) -> i64 {
  let mut total_distance = 0;
  for i in 0..galaxies.len() {
    for j in i + 1..galaxies.len() {
      let y = (galaxies[i].0 as i64 - galaxies[j].0 as i64).abs();
      let x = (galaxies[i].1 as i64 - galaxies[j].1 as i64).abs();
      total_distance += x + y;
    }
  }
  return total_distance;
}

fn map_galaxies(grid: &Grid<char>, empty_rows: Vec<usize> , empty_colums: Vec<usize> ) -> Vec<Location> {
  let mut galaxies = vec![];
  let great_divide = 999_999;
  let mut row_offset = 0;
  for (ri, row) in grid.iter_rows().enumerate() {
    if empty_rows.contains(&ri){
      row_offset += great_divide
    }
    let mut col_offset = 0;
    for (ci, c) in row.enumerate() {
      if empty_colums.contains(&ci){
        col_offset += great_divide;
      }
      if c == &'#' {
        galaxies.push((ri+row_offset, ci+col_offset))
      }
    }
  }

  return galaxies;
}

fn get_empty_rows(grid: &mut Grid<char>) ->Vec<usize> {
  grid
    .iter_rows()
    .enumerate()
    .filter_map(|(index, mut col)| {
      if col.by_ref().any(|c| c != &'.') {
        return None;
      }
      return Some(index);
    })
    .collect::<Vec<_>>()
}

fn get_empty_colums(grid: &mut Grid<char>) -> Vec<usize>{
  grid
    .iter_cols()
    .enumerate()
    .filter_map(|(index, mut row)| {
      if row.by_ref().any(|c| c != &'.') {
        return None;
      }
      return Some(index);
    })
    .collect::<Vec<_>>()
}

fn print_grid(grid: &Grid<char>) {
  for row in grid.iter_rows() {
    for c in row {
      print!("{c}");
    }
    println!();
  }
}
