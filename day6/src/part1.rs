pub(crate) fn part1(file: &String) {
  //242=p*(50-p)
  let one = 45-6;
  //1017=p*(74-p)
  let two = 56-19;
  //1691=p*(86-p)
  let three = 56-31;
  //1252=p*(85-p)
  let four = 67-19;
  println!("{}",one*two*three*four);
}

