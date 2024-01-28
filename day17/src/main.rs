use std::{fs, time::Instant};

// mod part1;
mod part2;
mod part2_2;

fn main() {
  let now = Instant::now();
  let file = read_file("./input.txt");
  // let file = read_file("./test.txt");
  // part1::part1(&file);
  // part2::part2(&file);
  part2_2::part_2(&file);

  println!("found answer in {:0.2?}", now.elapsed());
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
