use std::vec;

use indexmap::IndexMap;



pub(crate) fn part2(file: &String) {
  let steps = steps(file);

  let boxes = initialize(steps);

  let focusing_power = boxes.iter().enumerate().fold(0, |b_acc, (bi, boks)|{
    let boks_power = boks.values().enumerate().fold(0, |l_acc, (li, lens)| {
      let lens_power = (bi + 1) as i64 * (li+1) as i64 * *lens as i64;
      // println!("power: {lens_power}");
      l_acc + lens_power
    });
    b_acc + boks_power
  });

  println!("{boxes:?}");
  println!("{focusing_power:?}");
}

fn initialize(steps: Vec<Step>) -> Vec<IndexMap<String, u8>>{
  let mut boxes: Vec<IndexMap<String, u8>> = vec![IndexMap::new();256];
  for step in steps{
    let boks = &mut boxes[step.box_nr as usize];
    match step.operation{
        Operation::Remove => {
          println!("removing: {step:?}");
          boks.shift_remove(&step.label);
        },
        Operation::Insert(focal_length) => {
          println!("insert/updating: {step:?}");
          boks.entry(step.label).and_modify(|entry| *entry = focal_length).or_insert(focal_length);
        },
    }
  }

  return boxes;

}

#[derive(Debug)]
enum Operation{
  Remove,
  Insert(u8),
}

#[derive(Debug)]
struct Step{
  box_nr: u8,
  label: String,
  operation: Operation
}



fn steps(line: &String) -> Vec<Step> {
  line
    .split(',')
    .map(|command| {
      if command.ends_with('-'){
        let label = command[..command.len()-1].to_owned();
        let box_nr = hash(&label);
        let operation = Operation::Remove;
        return Step{box_nr, label, operation}
      }else{
        let split: Vec<&str> = command.split('=').collect();
        let label = split[0].to_owned();
        let box_nr = hash(&label);
        let operation = Operation::Insert(split[1].parse::<u8>().unwrap());
        return Step{box_nr, label, operation}
      }
    })
    .collect()
}

fn hash(label: &String) -> u8 {
  label.chars().map(|c| c as i32).fold(0, |mut acc, c|{
    acc += c;
    acc *= 17;
    acc %= 256;
    acc
  }) as u8
}