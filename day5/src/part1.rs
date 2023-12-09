pub(crate) fn part1(file: &String) {
  // parse mapping table to mapping functions which we can apply on the seeds.
  let mut line_iter = file.lines();
  let seeds = line_iter
    .next()
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .trim()
    .split(" ")
    .map(|nr| nr.parse::<i64>().unwrap())
    .collect::<Vec<_>>();
  line_iter.next(); // skip empty line.

  let mut maps: Vec<Map> = vec![];
  let mut map = Map::new();
  for (i, line) in line_iter.enumerate() {
    println!("line: {line}");
    if line.is_empty() {
      // create new map
      let mut swap = Map::new();
      std::mem::swap(&mut map, &mut swap);
      maps.push(swap);
      println!("added mapping to maps at line: {i}");
    } else if line.chars().nth(0).unwrap().is_ascii_digit() {
      // add mapping to map
      let mapping = line.split(" ").collect::<Vec<_>>();
      map.add(Mapping {
        destination: mapping[0].parse::<i64>().unwrap(),
        source: mapping[1].parse::<i64>().unwrap(),
        length: mapping[2].parse::<i64>().unwrap(),
      });
    }else{
      println!("skipping line: {line}");
    }
  }
  maps.push(map);

  println!("{:?}", lowest_location(&seeds, &maps));
}

fn lowest_location(seeds: &Vec<i64>, maps: &Vec<Map>) -> i64 {
  seeds
    .iter()
    .map(|seed| {
      let mut val: i64 = *seed;
      for map in maps {
        val = map.map(val)
      }
      return val;
    })
    .min()
    .unwrap()
}

#[derive(Debug)]
struct Map {
  mappings: Vec<Mapping>,
}

impl Map {
  pub fn new() -> Map {
    return Map { mappings: vec![] };
  }

  pub fn add(&mut self, mapping: Mapping) {
    self.mappings.push(mapping);
  }

  pub fn map(&self, val: i64) -> i64 {
    for mapping in self.mappings.iter() {
      if val > mapping.source && val < mapping.source + mapping.length {
        let offset = val - mapping.source;
        let new_val = mapping.destination + offset;
        // println!("mapping from {val} to {new_val}");
        return new_val;
      }
    }
    // println!("mapping from {val} to {val}");
    return val;
  }
}

    #[derive(Debug)]
    struct Mapping {
      source: i64,
      destination: i64,
      length: i64,
    }
