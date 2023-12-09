use std::ops::Range;
use std::ops::RangeBounds;
use std::vec;

use itertools::Itertools;
use range_ext::intersect::Intersect;
use range_ext::intersect::IntersectionExt::*;

pub(crate) fn part2(file: &String) {
  // parse mapping table to mapping functions which we can apply on the seeds.
  let mut line_iter = file.lines();
  let seeds_ranges = line_iter
    .next()
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .trim()
    .split(" ")
    .tuples::<(_, _)>()
    .map(|(start, length)| {
      let start = start.parse::<i64>().unwrap();
      let end = start + length.parse::<i64>().unwrap();
      start..end
    })
    .collect::<Vec<_>>();
  line_iter.next(); // skip empty line.

  let mut maps: Vec<Map> = vec![];
  let mut map = Map::new();
  for (i, line) in line_iter.enumerate() {
    // println!("line: {line}");
    if line.is_empty() {
      // create new map
      let mut swap = Map::new();
      std::mem::swap(&mut map, &mut swap);
      maps.push(swap);
      println!("added mapping to maps at line: {i}");
    } else if line.chars().nth(0).unwrap().is_ascii_digit() {
      // add mapping to map
      let mapping = line.split(" ").collect::<Vec<_>>();
      map.add(Mapping::new(
        mapping[1].parse::<i64>().unwrap(),
        mapping[0].parse::<i64>().unwrap(),
        mapping[2].parse::<i64>().unwrap(),
      ));
    } else {
      // println!("skipping line: {line}");
    }
  }
  maps.push(map);

  // add a mapping level to the seeds ranges.
  // apply a seed range to the level.
  // This returns 1 or more ranges for the next level
  // Add these to the top of the stack again
  // continue until last level is reached, then compare the min range value with the answer to find the lowest value.
  let mut results = vec![];
  for seed in seeds_ranges {
    results.push(MapResult {
      seed_range: seed.clone(),
      mapped_range: seed.clone(),
    })
  }

  println!("mapping seed-to-soil");
  results = maps[0].map(results);
  print_map_result(&results);
  println!("mapping soil-to-fertilizer");
  results = maps[1].map(results);
  print_map_result(&results);
  println!("mapping fertilizer-to-water");
  results = maps[2].map(results);
  print_map_result(&results);
  println!("mapping water-to-light");
  results = maps[3].map(results);
  print_map_result(&results);
  println!("mapping light-to-temperature");
  results = maps[4].map(results);
  print_map_result(&results);
  println!("mapping temperature-to-humidity");
  results = maps[5].map(results);
  print_map_result(&results);
  println!("mapping humidity-to-location");
  results = maps[6].map(results);
  print_map_result(&results);

  let min_seed = results
    .iter()
    .map(|result| (result.seed_range.start, result.mapped_range.start))
    .min_by(|r1, r2| r1.1.cmp(&r2.1))
    .unwrap()
    .1;
  println!("{min_seed:?}");
}

fn print_map_result(results: &Vec<MapResult>) {
  for result in results {
    let seed_len = result.seed_range.try_len().unwrap();
    let map_len = result.mapped_range.try_len().unwrap();
    if seed_len != map_len{panic!("ranges not equal!")}
    println!(
      "Seeds range:{:?} [{}] mapped to: {:?} [{}]",
      result.seed_range,
      seed_len,
      result.mapped_range,
      map_len
    )
  }
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

  pub fn map(&self, input: Vec<MapResult>) -> Vec<MapResult> {
    let (mut mapped, mut pass) = self
      .mappings
      .iter()
      .fold((vec![], input), |mut acc, mapping| {
        let mut mapped = vec![];
        let mut pass = vec![];

        for result in acc.1 {
          let (m, mut p) = mapping.map(result);
          if let Some(m) = m {
            mapped.push(m)
          }
          pass.append(&mut p)
        }
        mapped.append(&mut acc.0);

        (mapped, pass)
      });

    mapped.append(&mut pass);
    return mapped;
  }
}

#[derive(Debug)]
struct Mapping {
  range: Range<i64>,
  destination_offset: i64,
}

impl Mapping {
  fn new(start_source: i64, start_destination: i64, length: i64) -> Mapping {
    let mapping = Mapping {
      range: start_source..(start_source + length),
      destination_offset: start_destination - start_source,
    };
    println!("Created mapping: {:?}", mapping);
    return mapping;
  }

  // Map a range on a range. could result in 1-3 ranges
  pub fn map(&self, incomming: MapResult) -> (Option<MapResult>, Vec<MapResult>) {
    let intersection = incomming.mapped_range.intersect_ext(&self.range);
    match intersection {
      Empty => {
        println!("empty");
        return (None, vec![]);
      }
      Less | Greater => {
        println!("less or greater");
        // not overlapping, values remain the same
        return (None, vec![incomming]);
      }
      Within | Same => {
        println!("within or same");
        // all values of incomming range are mapped using the mapping.
        return (
          Some(MapResult {
            seed_range: incomming.seed_range,
            mapped_range: self.map_range(&incomming.mapped_range),
          }),
          vec![],
        );
      }
      LessOverlap => {
        println!("lessoverlap");
        // incomming overlaps start of mapping. split incomming range in 2 based on the start of the mapping
        // then first range remains the same, second range is mapped using mapping.
        let mut parts = incomming.split(self.range.start);
        parts[1].translate(self.destination_offset);
        return (Some(parts.remove(1)), vec![parts.remove(0)]);
      }
      GreaterOverlap => {
        println!("greateroverlap");
        // incomming overlaps end of mapping. split incomming range in 2 based on the end of the mapping
        // then first range is mapped using mapping, second range remains the same.
        let mut parts = incomming.split(self.range.end);
        parts[0].translate(self.destination_offset);
        return (Some(parts.remove(0)), vec![parts.remove(0)]);
      }
      Over => {
        println!("over");
        // the first range starts before and ends after mapping range. Split into 3 parts based on start and end of mapping
        // the first and third part remain the same. the second part is mapped using the mapping.
        let mut parts = incomming.split(self.range.start);
        let mut last_parts = parts.pop().unwrap().split(self.range.end);
        parts.append(&mut last_parts);
        parts[1].translate(self.destination_offset);
        return (
          Some(parts.remove(1)),
          vec![parts.remove(0), parts.remove(0)],
        );
      }
    }
  }

  fn map_range(&self, range: &Range<i64>) -> Range<i64> {
    Range {
      start: range.start + self.destination_offset,
      end: range.end + self.destination_offset,
    }
  }
}

#[derive(Debug, Clone)]
struct MapResult {
  seed_range: Range<i64>,
  mapped_range: Range<i64>,
}

impl MapResult {
  // split on a number in the range of the mapping
  fn split(self, split: i64) -> Vec<MapResult> {
    let seed_split = (split - self.mapped_range.start) + self.seed_range.start;
    println!("splitting range: {:?} at {split}", self.mapped_range);
    vec![
      MapResult {
        seed_range: self.seed_range.start..seed_split,
        mapped_range: self.mapped_range.start..split,
      },
      MapResult {
        seed_range: seed_split..self.seed_range.end,
        mapped_range: split..self.mapped_range.end,
      },
    ]
  }
  fn translate(&mut self, translation: i64) {
    self.mapped_range =
      (self.mapped_range.start + translation)..(self.mapped_range.end + translation)
  }
}
