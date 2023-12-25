use std::mem;

use itertools::Itertools;



pub(crate) fn part1(file: &String) {
  // read line
  // split on space
  // cast to vec<i32>
  // create Predictor 
  let mut predictors = file.lines().map(|line| { 
    let sequence = line.split(" ").map(|nr| nr.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    Predictor::new(sequence)
  }).collect::<Vec<_>>();

  let sum = predictors.iter_mut().map(|p|p.predict()).sum::<i32>();

  println!("{sum}");
}

#[derive(Debug)]
pub struct Predictor{
  sequences: Vec<Vec<i32>>
}

impl Predictor{
  fn new(sequence: Vec<i32>)-> Predictor{
    let mut sequences = vec![];
    let mut diff = sequence;
    let mut cont = true;
    while cont{
      let mut next = diff.iter().tuple_windows().map(|(l,r)|r-l).collect::<Vec<_>>();
      cont = next.iter().any(|v| *v!=0);
      mem::swap(&mut diff, &mut next);
      sequences.push(next);
    }
    sequences.push(diff);
    
    return Predictor{sequences}
  }

  fn predict(&mut self)-> i32{
    // iterate over arrays from second last to first
    // grab last value from previous array add it to last value from current array
    // add that value to current array.
    self.sequences.last_mut().unwrap().push(0);
    for i in (0..self.sequences.len() - 1).rev(){
      let prev = self.sequences[i+1].last().unwrap().clone();
      let last = self.sequences[i].last().unwrap().clone();
      self.sequences[i].push(prev+last);
    }
    return self.sequences[0].last().unwrap().clone();
  }
}