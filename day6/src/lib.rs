use utility::IntoArray;

const LENGTH: usize = 300;

fn input() -> [usize; LENGTH] {
  include_str!("../../inputs/day6.txt").trim_end().split(',').map(|n| n.parse().unwrap()).into_array()
}

pub fn solve_a() -> u32 {
  let mut fish = input().into_iter().fold([0u32; 9], |mut ages, n| {
    ages[n] += 1;
    ages
  });

  for d in 1..80 {
    fish[(d + 7) % 9] += fish[d % 9];
  }

  fish.iter().sum()
}

pub fn solve_b() -> u64 {
  let mut fish = input().into_iter().fold([0u64; 9], |mut ages, n| {
    ages[n] += 1;
    ages
  });

  for d in 1..256 {
    fish[(d + 7) % 9] += fish[d % 9];
  }

  fish.iter().sum()
}
