pub(crate) fn part1(file: String) {
  let sum: i32 = file
    .lines()
    .map(|line| {
      // Not very efficient.
      let mut calibration = String::from("");
      // find first number
      let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
      calibration.push(line.chars().nth(first).unwrap());
      // find last number
      let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
      calibration.push(line.chars().nth(last).unwrap());
      calibration.parse::<i32>().unwrap()
    })
    .sum();

  println!("{sum}");
}
