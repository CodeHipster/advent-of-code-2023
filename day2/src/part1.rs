
pub(crate) fn part1(file: &String) {
  let sum: i32 = file
    .lines()
    .map(|line| {
      let game_split = line.split(":").collect::<Vec<_>>();
      let game_nr = game_split[0]
        .trim()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

      let invalid = game_split[1]
        .split(";")
        .any(|hand| {
          hand.split(",").map(|cubes| cubes.trim()).any(|cubes| {
            let cube_count = cubes.split(" ").collect::<Vec<_>>();
            let color = cube_count[1];
            let count = cube_count[0].parse::<i32>().unwrap();
            // 0 = nr of cubes
            // 1 = color of cubes
            match color {
              "red" => count > 12,
              "green" => count > 13,
              "blue" => count > 14,
              _other => panic!("Should not contain another color.")
            }
          })
        });
        (game_nr, invalid)
    })
    // filter out invalid games
    .filter(|(_, valid)|!*valid)
    .map(|(game_nr,_)|game_nr)
    .sum();

  println!("{sum}");
}
