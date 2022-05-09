fn main() {
  println!("|--------|----------|---------------|");
  println!("| {0:^6} | {1:^8} | {2:^13} |", "DAY", "ANSWER A", "ANSWER B");
  println!("|--------|----------|---------------|");
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 1", day1::solve_a(), day1::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 2", day2::solve_a(), day2::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 3", day3::solve_a(), day3::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 4", day4::solve_a(), day4::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 5", day5::solve_a(), day5::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 6", day6::solve_a(), day6::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 7", day7::solve_a(), day7::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 8", day8::solve_a(), day8::solve_b());
  println!("| {0:^6} | {1:<8} | {2:<13} |", "Day 9", day9::solve_a(), day9::solve_b());
  println!("|--------|----------|---------------|");
}
