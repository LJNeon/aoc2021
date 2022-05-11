use utility::IntoArray;

const LENGTH: usize = 102;

fn input<'a>() -> [&'a str; LENGTH] {
  include_str!("../../inputs/day10.txt").lines().into_array()
}

fn equals(a: char, b: char) -> bool {
  match a {
    '(' => b == ')',
    '[' => b == ']',
    '{' => b == '}',
    '<' => b == '>',
    _ => unreachable!()
  }
}

pub fn solve_a() -> u32 {
  input()
    .into_iter()
    .map(|line| {
      let mut chunks = Vec::new();

      for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
          chunks.push(c);
        } else if equals(chunks[chunks.len() - 1], c) {
          chunks.pop();
        } else {
          return match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!()
          };
        }
      }

      0
    })
    .sum()
}

pub fn solve_b() -> u64 {
  let mut results: Vec<u64> = input()
    .into_iter()
    .map(|line| {
      let mut chunks = Vec::new();

      for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
          chunks.push(c);
        } else if equals(chunks[chunks.len() - 1], c) {
          chunks.pop();
        } else {
          return 0;
        }
      }

      chunks.into_iter().rev().fold(0, |total, c| {
        (total * 5)
          + match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!()
          }
      })
    })
    .filter(|&n| n != 0)
    .collect();

  results.sort_unstable();

  results[results.len() / 2]
}
