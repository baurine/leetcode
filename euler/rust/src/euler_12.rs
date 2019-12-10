/*
https://projecteuler.net/problem=12

What is the value of the first triangle number to have over five hundred divisors?
*/

fn divisors(n: u64) -> u64 {
  let mut count = 0u64;
  for i in 1..=n {
    if i * i > n {
      break;
    }
    if n % i == 0 {
      count += if n / i != i {
        2
      } else {
        1
      }
    }
  }
  count
}

pub fn solution() -> u64 {
  let mut accu = 0u64;
  for i in 1.. {
    accu += i;
    if divisors(accu) > 500 {
      break;
    }
  }
  accu
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 76576500);
  }
}
