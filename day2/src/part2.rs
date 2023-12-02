use std::{collections::HashMap, cmp::max};

pub(crate) fn part2(file: &String) {
  let sum: i32 = file
    .lines()
    .map(|line| {
      let game_split = line.split(":").collect::<Vec<_>>();

      let cubes = game_split[1]
        .split(";")
        .fold(HashMap::new(), |mut acc, hand| {
          hand.split(",").map(|cubes| cubes.trim()).for_each(|cubes| {
            let cube_count = cubes.split(" ").collect::<Vec<_>>();
            let color = cube_count[1];
            let count = cube_count[0].parse::<i32>().unwrap();
            let entry = acc.entry(color).or_insert(0);
            *entry = max(count, *entry);
          });
          return acc;
        });

      println!("{:?}", cubes);
      return cubes;
    })
    .fold(0, |acc, hand| {
      let power = hand.iter().fold(1, |acc, (_, count)| count*acc);
      power + acc
    });

  println!("{sum}");
}
