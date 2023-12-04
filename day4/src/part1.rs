use itertools::Itertools;

pub(crate) fn part1(file: &String) {
  //split on :
  // discard first part
  // split on |
  // 1 is winning numbers
  // 2 is your numbers
  //turn into vec<i32>
  // apply retain
  // power of 2 on the len()
  // sum()

  let sum: i32 = file
    .lines()
    .map(|line| {
      let nrs = line
        .split(":")
        .nth(1)
        .unwrap()
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
          if count == 0 {
            count = 1
          } else {
            count *= 2;
          }
        }
      }
      count
    })
    .sum();

  println!("{sum}");
}
