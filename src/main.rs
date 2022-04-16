use day1::*;
use day2::*;
use utility::Solvable;

fn main() {
  println!("|----------|------------|------------|");
  println!("| {0: <8} | {1: <10} | {2: <10} |", "DAY", "ANSWER A", "ANSWER B");
  println!("|----------|------------|------------|");
  let one = Day1::new();
  println!("| {0: <8} | {1: <10} | {2: <10} |", Day1::name(), one.solve_a(), one.solve_b());
  let two = Day2::new();
  println!("| {0: <8} | {1: <10} | {2: <10} |", Day2::name(), two.solve_a(), two.solve_b());
  println!("|----------|------------|------------|");
  println!("\nDone!");
}
