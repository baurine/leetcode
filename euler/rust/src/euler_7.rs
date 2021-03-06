/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/
use crate::math_util;

pub fn solution() -> u32 {
  let mut order = 1;
  for i in (3..).step_by(2) {
    if math_util::is_prime(i as u64) {
      order += 1;
      println!("{}st: {}", order, i);
      if order == 10_001 {
        return i;
      }
    }
  }
  0
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(solution(), 104743);
  }
}
