pub trait Solvable {
  fn new() -> Self;
  fn name() -> &'static str;
  fn solve_a(&self) -> String;
  fn solve_b(&self) -> String;
}
