use grid::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;

pub struct Part {
  pub number: i32,
  pub border: Vec<Symbol>,
}

pub struct Symbol{
  pub char: char,
  pub row: usize,
  pub col: usize,
}

pub struct Schematic {
  pub grid: Grid<char>,
}

impl Schematic {
  pub fn new(file: &String) -> Schematic {
    let mut schema = Schematic { grid: grid![] };

    file.lines().for_each(|line| {
      schema.add_row(line);
    });
    return schema;
  }

  fn add_row(&mut self, line: &str) {
    let chars: Vec<char> = line.chars().collect();
    self.grid.push_row(chars);
  }

  // end is index of char after number
  fn get_border(&self, row: usize, start: usize, end: usize) -> Vec<Symbol> {
    let mut border: Vec<Symbol> = vec![];
    let start_col = if start == 0 { 0 } else { start - 1 };
    for col in start_col..=end {
      if row > 0 {
        // previous row if it exists
        if let Some(value) = Schematic::filter_symbol(self.grid.get(row - 1, col)) {
          border.push(Symbol{char:value.clone(), col, row: row-1});
        }
      }
      // bottom row
      if let Some(value) = Schematic::filter_symbol(self.grid.get(row + 1, col)) {
        border.push(Symbol{char:value.clone(), col, row: row+1});
      }
    }

    if start > 0 {
      // previous col if it exists
      if let Some(value) = Schematic::filter_symbol(self.grid.get(row, start - 1)) {
        border.push(Symbol{char:value.clone(), col: start-1, row});
      }
    }
    // the character after the number
    if let Some(value) = Schematic::filter_symbol(self.grid.get(row, end)) {
      border.push(Symbol{char:value.clone(), col: end, row: row});
    }
    return border;
  }

  fn filter_symbol(c: Option<&char>) -> Option<&char> {
    match c {
      None => None,
      Some('.') => None,
      Some(c) if c.is_ascii_digit() => panic!("border can't contain numbers"),
      Some(c) => Some(c),
    }
  }

  pub fn get_parts(&self) -> Vec<Part> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    self
      .grid
      .iter_rows()
      .enumerate()
      .flat_map(|(row_i, row)| {
        let row_str = row.collect::<String>();
        RE.find_iter(&row_str)
          .map(|mtch| (row_i, mtch.start(), mtch.end(), mtch.as_str().to_owned()))
          .collect::<Vec<_>>()
      })
      .map(|(row, start, end, nr)| {
        // get the surrounding chars
        let part = Part {
          number: nr.parse::<i32>().unwrap(),
          border: self.get_border(row, start, end),
        };
        return part;
      })
      .collect()
  }
}

impl Display for Schematic {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in 0..self.grid.rows() {
      for col in 0..self.grid.cols() {
        write!(f, "{}", self.grid.get(row, col).unwrap())?;
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}

impl Display for Part {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{} {:?}", self.number, self.border.iter().map(|b|b.to_string()).collect::<Vec<_>>().join("','"))
  }
}

impl Display for Symbol{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} [{},{}]", self.char, self.row, self.col)
  }
}
