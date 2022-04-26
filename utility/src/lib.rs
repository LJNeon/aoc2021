pub trait IntoArray: IntoIterator {
  fn into_array<const N: usize>(self) -> [Self::Item; N];
}

impl<I: IntoIterator> IntoArray for I {
  fn into_array<const N: usize>(self) -> [Self::Item; N] {
    let mut iter = self.into_iter();

    [(); N].map(|_| iter.next().unwrap())
  }
}
