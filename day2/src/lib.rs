use utility::IntoArray;

const LENGTH: usize = 1000;

#[derive(Debug, Clone, Copy)]
pub enum Move {
  Forward(u8),
  Up(u8),
  Down(u8)
}

fn input() -> [Move; LENGTH] {
  include_str!("../../inputs/day2.txt")
    .lines()
    .map(|l| l.split_once(' ').unwrap())
    .map(|(d, i)| match (d, i.parse::<u8>().unwrap()) {
      ("forward", i) => Move::Forward(i),
      ("up", i) => Move::Up(i),
      ("down", i) => Move::Down(i),
      _ => unreachable!()
    })
    .into_array()
}

pub fn solve_a() -> u32 {
  let pos = input().iter().fold((0, 0), |(h, d), m| match m {
    Move::Forward(i) => (h + (*i as u32), d),
    Move::Up(i) => (h, d - (*i as u32)),
    Move::Down(i) => (h, d + (*i as u32))
  });

  pos.0 * pos.1
}

pub fn solve_b() -> u32 {
  let pos = input().iter().fold((0, 0, 0), |(h, d, a), m| match m {
    Move::Forward(i) => (h + (*i as u32), d + (a * (*i as u32)), a),
    Move::Up(i) => (h, d, a - (*i as u32)),
    Move::Down(i) => (h, d, a + (*i as u32))
  });

  pos.0 * pos.1
}
