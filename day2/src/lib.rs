use once_cell::sync::Lazy;

pub enum Move {
  Forward(u32),
  Up(u32),
  Down(u32)
}

static MOVES: Lazy<Vec<Move>> = Lazy::new(|| {
  include_str!("../input.txt")
    .lines()
    .map(|l| l.split_once(' ').unwrap())
    .map(|(d, i)| match (d, i.parse::<u32>().unwrap()) {
      ("forward", i) => Move::Forward(i),
      ("up", i) => Move::Up(i),
      ("down", i) => Move::Down(i),
      _ => unreachable!()
    })
    .collect()
});

pub fn solve_a() -> u32 {
  let pos = MOVES.iter().fold((0, 0), |(h, d), m| match m {
    Move::Forward(i) => (h + i, d),
    Move::Up(i) => (h, d - i),
    Move::Down(i) => (h, d + i)
  });

  pos.0 * pos.1
}

pub fn solve_b() -> u32 {
  let pos = MOVES.iter().fold((0, 0, 0), |(h, d, a), m| match m {
    Move::Forward(i) => (h + i, d + (a * i), a),
    Move::Up(i) => (h, d, a - i),
    Move::Down(i) => (h, d, a + i)
  });

  pos.0 * pos.1
}
