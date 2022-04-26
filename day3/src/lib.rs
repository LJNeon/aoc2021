use utility::IntoArray;

const WIDTH: usize = 12;
const LENGTH: usize = 1000;

fn input() -> [u32; LENGTH] {
  include_str!("../../inputs/day3.txt").lines().map(|l| u32::from_str_radix(l, 2).unwrap()).into_array()
}

fn get_bit(n: &u32, i: usize) -> u32 {
  (n & (1 << i)) >> i
}

pub fn solve_a() -> u32 {
  let mut count = vec![0; WIDTH];
  let total = (LENGTH / 2) as u32;

  for num in input() {
    for i in 0..WIDTH {
      count[i] += get_bit(&num, i);
    }
  }

  let gamma: u32 = count.into_iter().enumerate().map(|(i, n)| ((n >= total) as u32) << i).sum();

  gamma * (!gamma & ((1 << WIDTH) - 1))
}

fn search(list: &mut Vec<u32>, i: usize, gt: bool) {
  if list.len() == 1 {
    return;
  }

  let which = ((list.iter().filter(|n| get_bit(n, i) == 1).count() >= (list.len() + 1) / 2) == gt) as u32;

  list.retain(|n| get_bit(n, i) == which);
}

pub fn solve_b() -> u32 {
  let mut generator = input().to_vec();
  let mut scrubber = generator.clone();

  for i in (0..WIDTH).rev() {
    search(&mut generator, i, true);
    search(&mut scrubber, i, false);
  }

  generator[0] * scrubber[0]
}
