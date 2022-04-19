use once_cell::sync::Lazy;

const WIDTH: usize = 12;
const THRESHOLD: u32 = 500;

static NUMBERS: Lazy<Vec<u32>> = Lazy::new(|| {
  include_str!("../input.txt").lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect()
});

fn get_bit(n: &u32, i: usize) -> u32 {
  (n & (1 << i)) >> i
}

pub fn solve_a() -> u32 {
  let mut count = vec![0; WIDTH];

  for num in NUMBERS.iter() {
    for i in 0..WIDTH {
      count[i] += get_bit(num, i);
    }
  }

  let gamma: u32 = count.into_iter().enumerate().map(|(i, n)| ((n >= THRESHOLD) as u32) << i).sum();

  gamma * (!gamma & ((1 << WIDTH) - 1))
}

fn count(vec: &Vec<u32>, i: usize) -> (u32, u32) {
  let mut result: (u32, u32) = (0, 0);

  for num in vec {
    result.1 += 1;
    result.0 += get_bit(num, i);
  }

  result
}

pub fn solve_b() -> u32 {
  let (mut generator, mut scrubber) = (NUMBERS.clone(), NUMBERS.clone());

  for i in (0..WIDTH).rev() {
    let mut amount = count(&generator, i);

    if amount.1 > 1 {
      let which = (amount.0 >= amount.1 / 2) as u32;

      generator.retain(|n| get_bit(n, i) == which);
    }

    let generator_done = amount.1 == 1;

    amount = count(&scrubber, i);

    if amount.1 > 1 {
      let which = (amount.0 < amount.1 / 2) as u32;

      scrubber.retain(|n| get_bit(n, i) == which);
    } else if generator_done {
      break;
    }
  }

  generator[0] * scrubber[0]
}
