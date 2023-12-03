use crate::schematic::Schematic;


pub(crate) fn part1(file: &String) {
  // list part numbers with list of surroundings
  // load file into 2d map
  // for each line find numbers
  // for each number get start + length
  // get surrounding values from map.
  let schema = Schematic::new(file);
  println!("{schema}");
  let parts = schema.get_parts();
  let sum:i32 = parts.iter().filter(|part| !part.border.is_empty()).map(|part|part.number).sum();
  println!("{sum}")
}
