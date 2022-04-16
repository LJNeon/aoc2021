use once_cell::sync::Lazy;

static NUMBERS: Lazy<Vec<u32>> = Lazy::new(|| {
  include_str!("../input.txt").lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect()
});

pub fn solve_a() -> u32 {
  let width = NUMBERS.iter().map(|n| 32 - n.leading_zeros()).max().unwrap() as usize;
  let mut count = vec![0; width];

  for num in NUMBERS.iter() {
    for i in 0..width {
      count[i] += (num & (1 << i)) >> i;
    }
  }

  let threshold = (NUMBERS.len() / 2) as u32;
  let gamma: u32 = count.into_iter().enumerate().map(|(i, n)| ((n >= threshold) as u32) << i).sum();

  gamma * (!gamma & ((1 << width) - 1))
}

pub fn solve_b() -> String {
  "todo".to_string()
}
