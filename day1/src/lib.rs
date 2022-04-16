use once_cell::sync::Lazy;

static NUMBERS: Lazy<Vec<u16>> =
  Lazy::new(|| include_str!("../input.txt").lines().map(|n| n.parse().unwrap()).collect());

pub fn solve_a() -> usize {
  NUMBERS.windows(2).filter(|n| n[0] < n[1]).count()
}

pub fn solve_b() -> usize {
  NUMBERS.windows(4).filter(|n| n[0] < n[3]).count()
}
