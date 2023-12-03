use std::collections::HashMap;

use crate::schematic::Schematic;

pub(crate) fn part2(file: &String) {
  let schema = Schematic::new(file);
  println!("{schema}");
  let parts = schema.get_parts();
  let sum = parts
    .iter()
    .fold(HashMap::new(), |mut acc, part| {
      if let Some(symbol) = part.border.iter().find(|symbol| symbol.char == '*') {
        let entry = acc.entry((symbol.row, symbol.col)).or_insert(vec![]);
        entry.push(part.number);
      }
      return acc;
    })
    .iter()
    .fold(0, |acc, (_, part_nrs)| {
      if part_nrs.len() > 1 {
        return acc + (part_nrs[0] * part_nrs[1]);
      }
      return acc;
    });
  println!("{sum}");
}
