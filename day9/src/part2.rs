use std::{mem, collections::VecDeque, vec};

use itertools::Itertools;

pub(crate) fn part2(file: &String) {
  // read line
  // split on space
  // cast to vec<i32>
  // create Predictor 
  let mut predictors = file.lines().map(|line| { 
    let sequence = line.split(" ").map(|nr| nr.parse::<i32>().unwrap()).collect::<VecDeque<i32>>();
    Predictor::new(sequence)
  }).collect::<VecDeque<_>>();

  let sum = predictors.iter_mut().map(|p|p.historic()).sum::<i32>();

  println!("{sum}");
}

#[derive(Debug)]
pub struct Predictor{
  sequences: Vec<VecDeque<i32>>
}

impl Predictor{
  fn new(sequence: VecDeque<i32>)-> Predictor{
    let mut sequences:Vec<VecDeque<i32>> = vec![];
    let mut diff = sequence;
    let mut cont = true;
    while cont{
      let mut next = diff.iter().tuple_windows().map(|(l,r)|r-l).collect::<VecDeque<_>>();
      cont = next.iter().any(|v| *v!=0);
      mem::swap(&mut diff, &mut next);
      sequences.push(next);
    }
    sequences.push(diff);
    
    return Predictor{sequences}
  }

  fn historic(&mut self)-> i32{

    // iterate over arrays from second last to first
    // grab first value from previous array subtract it from first value from current array
    // add that value to front of current array.
    self.sequences.last_mut().unwrap().push_front(0);
    for i in (0..self.sequences.len() - 1).rev(){
      let prev = self.sequences[i+1].front().unwrap().clone();
      let last = self.sequences[i].front().unwrap().clone();
      self.sequences[i].push_front(last-prev);
    }
    return self.sequences[0].front().unwrap().clone();
  }

  fn predict(&mut self)-> i32{
    // iterate over arrays from second last to first
    // grab last value from previous array add it to last value from current array
    // add that value to current array.
    self.sequences.last_mut().unwrap().push_back(0);
    for i in (0..self.sequences.len() - 1).rev(){
      let prev = self.sequences[i+1].back().unwrap().clone();
      let last = self.sequences[i].back().unwrap().clone();
      self.sequences[i].push_back(prev+last);
    }
    return self.sequences[0].back().unwrap().clone();
  }
}