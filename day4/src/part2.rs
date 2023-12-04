use std::collections::HashMap;

use itertools::Itertools;

pub(crate) fn part2(file: &String) {
  // iterate over cards
  // maintain hashmap of nr of copies of each card
  // add cards to hashmap based on nr of matches
  // sum up values of hashmap

  let cards: HashMap<i32, i32> = file.lines().fold(HashMap::new(), |mut acc, line| {
    let game_nrs_split = line.split(":").collect::<Vec<_>>();
    let game_nr = &game_nrs_split[0][4..].trim().parse::<i32>().unwrap();

    // add the card of the line.
    let entree = acc.entry(*game_nr).or_insert(0);
    *entree += 1;
    let nr_of_cards = entree.clone();

    // get count of winning nrs
    let nrs = game_nrs_split[1]
      .split("|")
      .map(|nrs| {
        nrs
          .chars()
          .tuples::<(_, _, _)>()
          .map(|nr| {
            let mut str_nr = String::new();
            str_nr.push(nr.1);
            str_nr.push(nr.2);
            str_nr.trim().parse::<i32>().unwrap()
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();

    let winning_nrs = &nrs[0];
    let my_nrs = &nrs[1];
    let mut count: i32 = 0;
    for nr in winning_nrs {
      if my_nrs.contains(nr) {
        count += 1;
      }
    }

    // add the winning cards to the hashmap
    for game in *game_nr..(*game_nr + count) {
      let to_add = game + 1;
      let entry = acc.entry(to_add).or_insert(0);
      *entry += nr_of_cards;
      println!("game: {game_nr}, adding {nr_of_cards} cards to: {to_add}");
    }
    acc
  });

  let sum = cards.iter().fold(0, |acc,(_,count)|acc + count);
  println!("{sum}");
}
