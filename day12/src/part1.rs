use std::collections::VecDeque;

#[derive(Debug)]
struct State {
  pattern: String,
  groups: VecDeque<usize>,
}

enum Node {
  Branch(VecDeque<State>),
  Leaf(State),
  Invalid,
}

pub(crate) fn part1(file: &String) {
  // figure out a way to test nr of combinations

  // with only 1 group, move group through the positions
  // ??? # -> #.. .#. ..#

  // with multiple groups
  // first group for each option, test all other options
  // ???? # # -> #.#. #..# .#.#

  // recursion? will only go as deep as nr of groups
  // manage state?
  // state:
  // - group order
  // - pattern
  // - for each group the location
  // loop while there is positions

  let possibilities: i32 = file
    .lines()
    .map(|line| {
      let split = line.split(" ").collect::<Vec<_>>();
      let pattern = split[0];
      let groups = split[1]
        .split(',')
        .map(|group| group.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

      let p = possibilities(pattern, groups);
      println!(
        "{p} possibilities for pattern: {pattern}, with groups: {}",
        split[1]
      );
      p
    })
    .sum();

  println!("{possibilities}");

  // It is basically traversing a tree?
  // first node is a pattern and groups without a location?
  // for the next nodes we pop the first group, place it on all possible locations
  // then for space and groups left over, create new nodes?
  // when do we count it as a possible pattern? if it returns none?
}

fn valid_state(pattern: &str, result:&str)-> bool{

  let pat_char = pattern.chars().collect::<Vec<_>>();
  let res_char = result.chars().collect::<Vec<_>>();

  // Result could be shorter than pattern, only check if the result so far is correct.
  // A branch has a partial pattern we want to validate to exit early.
  for i in 0 .. result.len(){
    match pat_char[i]{
      '?' => {
        continue;
      },
      c => if c != res_char[i]{ 
        // println!("result: {result}, does not match pattern: {pattern}");
        return false
      }
    }
  }

  return true;
}

fn possibilities(pattern: &str, groups: VecDeque<usize>) -> i32 {
  // pattern, list of groups and their max value
  let mut stack: VecDeque<State> = VecDeque::new();
  let initial_state = State {
    pattern: "".to_string(),
    groups,
  };

  stack.push_back(initial_state);

  let mut possibilities = 0;

  while !stack.is_empty() {
    let state = stack.pop_back().unwrap();

    match next(state, pattern) {
      Node::Branch(mut states) => stack.append(&mut states),
      Node::Leaf(_) => possibilities += 1,
      Node::Invalid => {}
    }
  }

  return possibilities;
}

// Get next states based on current state.
// if this is a leaf node, None is returned
// if there are no more options an empty vec is returned.
fn next(mut state: State, pattern: &str) -> Node {
  
  if !valid_state(pattern, &state.pattern){
    return Node::Invalid;
  }

  let max = pattern.len();
  if state.pattern.len() < max {
    if state.pattern.ends_with('#') {
      state.pattern += &".";
    }
  }

  if let Some(group) = state.groups.pop_front() {
    // get possible space the group can fill
    let space = max - state.pattern.len() - state.groups.iter().fold(0, |acc, len| acc + 1 + len);

    let patterns = patterns(space, group);

    return Node::Branch(
      patterns
        .iter()
        .map(|pat| State {
          pattern: state.pattern.clone() + pat,
          groups: state.groups.clone(),
        })
        .collect(),
    );
  } else {
    println!("leaf node: {state:?}");
    return Node::Leaf(state);
  }
}

// get possible patterns a group can fill in a space
fn patterns(length: usize, group: usize) -> Vec<String> {
  let mut patterns = vec![];
  let mut start = 0;
  while start + group <= length {
    let base = vec!['.'; start + group];
    let mut pattern = base.clone();
    for i in 0..group {
      pattern[i + start] = '#';
    }
    patterns.push(pattern.iter().collect());
    start += 1;
  }
  return patterns;
}
