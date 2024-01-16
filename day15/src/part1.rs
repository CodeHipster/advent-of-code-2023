pub(crate) fn part1(file: &String) {
  let commands = commands(file);

  let answer = commands.iter().fold(0, |mut acc, command|{
    acc += command.iter().fold(0, |mut acc, c|{
      acc += *c as i32;
      acc *= 17;
      acc %= 256;
      acc
    });
    acc
  });

  println!("{answer}");
}

fn commands(line: &String) -> Vec<Vec<u8>> {
  line
    .split(',')
    .map(|command| {
      command
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>()
    })
    .collect()
}
