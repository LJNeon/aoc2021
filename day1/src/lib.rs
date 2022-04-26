use utility::IntoArray;

const LENGTH: usize = 2000;

fn input() -> [u16; LENGTH] {
  include_str!("../../inputs/day1.txt").lines().map(|n| n.parse().unwrap()).into_array()
}

pub fn solve_a() -> usize {
  input().windows(2).filter(|n| n[0] < n[1]).count()
}

pub fn solve_b() -> usize {
  input().windows(4).filter(|n| n[0] < n[3]).count()
}
