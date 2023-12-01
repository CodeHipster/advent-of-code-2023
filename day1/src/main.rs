use std::fs::{File, self};

mod part1;
mod part2;

fn main() {
    let file = read_file("./input.txt");
    // part1::part1()
    part2::part2(file);
}


fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}