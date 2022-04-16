use std::str::FromStr;

use utility::Solvable;

fn to_digit(s: &str) -> u8 {
  s.chars().last().unwrap() as u8 - 0x30
}

pub enum Movement {
  Forward(u8),
  Up(u8),
  Down(u8)
}

impl FromStr for Movement {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Ok(match input.split_once(' ').unwrap_or_default() {
      ("forward", i) => Movement::Forward(to_digit(i)),
      ("up", i) => Movement::Up(to_digit(i)),
      ("down", i) => Movement::Down(to_digit(i)),
      _ => panic!("Failed to parse day2/input_a.txt")
    })
  }
}

pub struct Day2(Vec<Movement>);

impl Solvable for Day2 {
  fn new() -> Day2 {
    Day2(include_str!("../input.txt").lines().map(|x| x.parse().unwrap()).collect())
  }

  fn name() -> &'static str {
    "Day 2"
  }

  fn solve_a(&self) -> String {
    let pos = self.0.iter().fold((0u32, 0u32), |(h, d), x| match x {
      Movement::Forward(x) => (h + (*x as u32), d),
      Movement::Up(x) => (h, d - (*x as u32)),
      Movement::Down(x) => (h, d + (*x as u32))
    });

    (pos.0 * pos.1).to_string()
  }

  fn solve_b(&self) -> String {
    let pos = self.0.iter().fold((0u32, 0u32, 0u32), |(h, d, a), x| match x {
      Movement::Forward(x) => (h + (*x as u32), d + (a * (*x as u32)), a),
      Movement::Up(x) => (h, d, a - (*x as u32)),
      Movement::Down(x) => (h, d, a + (*x as u32))
    });

    (pos.0 * pos.1).to_string()
  }
}
