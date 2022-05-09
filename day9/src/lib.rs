use utility::IntoArray;

const LENGTH: usize = 100;

fn input() -> [[u8; LENGTH]; LENGTH] {
  include_str!("../../inputs/day9.txt").lines().map(|l| l.bytes().into_array()).into_array()
}

pub fn solve_a() -> u16 {
  let heightmap = input();
  let mut total = 0;

  for y in 0..heightmap.len() {
    for x in 0..heightmap[y].len() {
      if (x == 0 || heightmap[y][x - 1] > heightmap[y][x])
        && (y == 0 || heightmap[y - 1][x] > heightmap[y][x])
        && (x == heightmap[y].len() - 1 || heightmap[y][x + 1] > heightmap[y][x])
        && (y == heightmap.len() - 1 || heightmap[y + 1][x] > heightmap[y][x])
      {
        total += (heightmap[y][x] - b'0') as u16;
      }
    }
  }

  total
}

pub fn travel_basin(heightmap: &mut [[u8; LENGTH]], x: usize, y: usize) -> u32 {
  let mut total = 1;

  heightmap[y][x] = b'9';

  if x != 0 && heightmap[y][x - 1] != b'9' {
    total += travel_basin(heightmap, x - 1, y);
  }

  if y != 0 && heightmap[y - 1][x] != b'9' {
    total += travel_basin(heightmap, x, y - 1);
  }

  if x != heightmap[y].len() - 1 && heightmap[y][x + 1] != b'9' {
    total += travel_basin(heightmap, x + 1, y);
  }

  if y != heightmap.len() - 1 && heightmap[y + 1][x] != b'9' {
    total += travel_basin(heightmap, x, y + 1);
  }

  total
}

pub fn solve_b() -> u32 {
  let mut heightmap = input();
  let mut basins = Vec::new();

  for y in 0..heightmap.len() {
    for x in 0..heightmap[y].len() {
      if heightmap[y][x] != b'9' {
        basins.push(travel_basin(&mut heightmap, x, y));
      }
    }
  }

  basins.sort_unstable();

  basins.into_iter().rev().take(3).product()
}
