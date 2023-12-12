use std::{collections::HashMap, fmt::Display};

pub(crate) fn part2(file: &String) {
  let mut hands: Vec<_> = file
    .lines()
    .map(|line| {
      let input: Vec<&str> = line.split(" ").collect();
      let bid = input[1].parse::<i32>().unwrap();
      Hand::new(input[0], bid)
    })
    .collect();

  hands.sort();
  hands.iter().for_each(|hand| println!("{hand}"));

  let winnings = hands
    .iter()
    .rev()
    .fold((0, 1), |(winnings, rank), hand| {
      let win = hand.bid * rank;
      (winnings + win, rank + 1)
    })
    .0;

  println!("{winnings}");
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
  hand_type: HandType,
  cards: Vec<CardType>,
  bid: i32,
  input: String,
}

impl Hand {
  fn new(hand: &str, bid: i32) -> Hand {
    let hand_type = Hand::get_hand_type(hand);
    let cards = Hand::get_cards(hand);
    Hand {
      hand_type,
      bid,
      cards,
      input: hand.to_owned(),
    }
  }
  fn get_cards(hand: &str) -> Vec<CardType> {
    hand
      .chars()
      .map(|c| match c {
        'A' => CardType::A,
        'K' => CardType::K,
        'Q' => CardType::Q,
        'J' => CardType::J,
        'T' => CardType::T,
        '9' => CardType::Nine,
        '8' => CardType::Eight,
        '7' => CardType::Seven,
        '6' => CardType::Six,
        '5' => CardType::Five,
        '4' => CardType::Four,
        '3' => CardType::Three,
        '2' => CardType::Two,
        _ => panic!("unknown card: {c}"),
      })
      .collect()
  }

  fn get_hand_type(hand: &str) -> HandType {
    let mut set: HashMap<char, i8> =
      hand
        .chars()
        .fold(HashMap::<char, i8>::new(), |mut acc, card| {
          acc.entry(card).and_modify(|count| *count += 1).or_insert(1);
          acc
        });

    // find jokers and replace them with most occuring card.
    if let Some(count) = set.remove(&'J') {
      // if we have 5 jokers, we have five of a kind.
      if count == 5i8 {
        return HandType::FiveOfAKind;
      }
      // we have some jokers. Add them to the most frequent card.
      let max = set
        .iter()
        .max_by(|(_, count), (_, count2)| count.cmp(count2))
        .unwrap();
      set.entry(*max.0).and_modify(|e| *e += count);
    }

    match set.len() {
      1 => HandType::FiveOfAKind, // 5 of a kind
      2 => {
        // 4 of a kind or full house
        match set.values().max().unwrap() {
          3 => HandType::FullHouse,
          4 => HandType::FourOfAKind,
          _ => panic!("impossible hand: {:?}", hand),
        }
      }
      3 => {
        // 3 of a kind or 2 pair
        match set.values().max().unwrap() {
          2 => HandType::TwoPair,
          3 => HandType::ThreeOfAKind,
          _ => panic!("impossible hand: {:?}", hand),
        }
      }
      4 => HandType::Pair,     // pair
      5 => HandType::HighCard, // high card
      _ => panic!("what kind of hand did we get? {:?}", hand),
    }
  }
}

impl Display for Hand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {:?} {}", self.input, self.hand_type, self.bid)
  }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPair,
  Pair,
  HighCard,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CardType {
  A,
  K,
  Q,
  T,
  Nine,
  Eight,
  Seven,
  Six,
  Five,
  Four,
  Three,
  Two,
  J,
}
