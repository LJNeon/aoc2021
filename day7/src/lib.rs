use utility::IntoArray;

const LENGTH: usize = 1000;

fn input() -> [u32; LENGTH] {
  include_str!("../../inputs/day7.txt").trim_end().split(',').map(|n| n.parse().unwrap()).into_array()
}

pub fn solve_a() -> u32 {
  let mut crabs = input();
  crabs.sort_unstable();
  let median = crabs[crabs.len() / 2];

  crabs.iter().fold(0, |fuel, c| fuel + c.abs_diff(median) as u32)
}

pub fn solve_b() -> u32 {
  let crabs = input();

  (crabs.iter().sum::<u32>() / crabs.len() as u32..)
    .take(2)
    .map(|crab| {
      crabs.iter().fold(0, |fuel, c| {
        let diff = c.abs_diff(crab);

        fuel + ((diff * (diff + 1)) / 2)
      })
    })
    .min()
    .unwrap()
}
