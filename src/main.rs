fn main() {
  println!("|----------|------------|------------|");
  println!("| {0:^8} | {1:^10} | {2:^10} |", "DAY", "ANSWER A", "ANSWER B");
  println!("|----------|------------|------------|");
  println!("| {0:^8} | {1:<10} | {2:<10} |", "Day 1", day1::solve_a(), day1::solve_b());
  println!("| {0:^8} | {1:<10} | {2:<10} |", "Day 2", day2::solve_a(), day2::solve_b());
  println!("| {0:^8} | {1:<10} | {2:<10} |", "Day 3", day3::solve_a(), day3::solve_b());
  println!("| {0:^8} | {1:<10} | {2:<10} |", "Day 4", day4::solve_a(), day4::solve_b());
  println!("|----------|------------|------------|");
}
