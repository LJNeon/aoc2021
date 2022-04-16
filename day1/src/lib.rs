use utility::Solvable;

pub struct Day1(Vec<u16>);

impl Solvable for Day1 {
  fn new() -> Day1 {
    Day1(include_str!("../input.txt").lines().map(|x| x.parse().unwrap()).collect())
  }

  fn name() -> &'static str {
    "Day 1"
  }

  fn solve_a(&self) -> String {
    self
      .0
      .iter()
      .fold((&u16::MAX, 0), |(a, b), x| if a < x { (x, b + 1) } else { (x, b) })
      .1
      .to_string()
  }

  fn solve_b(&self) -> String {
    self
      .0
      .iter()
      .enumerate()
      .take_while(|&(i, _)| i < self.0.len() - 3)
      .fold(0, |acc, (i, x)| {
        let a = x + self.0[i + 1] + self.0[i + 2];
        let b = self.0[i + 1] + self.0[i + 2] + self.0[i + 3];

        if a < b {
          acc + 1
        } else {
          acc
        }
      })
      .to_string()
  }
}
