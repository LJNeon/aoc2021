use utility::IntoArray;

const LENGTH: usize = 200;

fn input<'a>() -> [&'a str; LENGTH] {
  include_str!("../../inputs/day8.txt").lines().into_array()
}

pub fn solve_a() -> usize {
  input().iter().fold(0, |total, l| {
    total + l[61..].split_whitespace().filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7).count()
  })
}

fn parse_digit(digit: &str, four: &str, seven: &str) -> u32 {
  match digit.len() {
    2 => 1,
    3 => 7,
    4 => 4,
    7 => 8,
    5 => {
      if seven.chars().all(|c| digit.contains(c)) {
        3
      } else if four.chars().filter(|&c| digit.contains(c)).count() == 2 {
        2
      } else {
        5
      }
    },
    6 => {
      if four.chars().all(|c| digit.contains(c)) {
        9
      } else if seven.chars().all(|c| digit.contains(c)) {
        0
      } else {
        6
      }
    },
    _ => unreachable!()
  }
}

pub fn solve_b() -> u32 {
  input().into_iter().fold(0, |total, line| {
    let (key, numbers) = line.split_once(" | ").unwrap();
    let four = key.split_whitespace().find(|n| n.len() == 4).unwrap();
    let seven = key.split_whitespace().find(|n| n.len() == 3).unwrap();

    total
      + numbers
        .split_whitespace()
        .enumerate()
        .map(|(i, n)| parse_digit(n, four, seven) * u32::pow(10, 3 - i as u32))
        .sum::<u32>()
  })
}
