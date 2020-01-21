pub fn square(s: u32) -> u64 {
  // if s > 64 || s < 1 {
  //   panic!("Square must be between 1 and 64");
  // }
  assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");
  2u64.pow(s - 1)
}

pub fn total() -> u64 {
  // (1..=64).map(|n| square(n)).sum::<u64>()
  square(64) - 1 + square(64)
}
