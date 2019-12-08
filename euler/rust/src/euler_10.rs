/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

// 使用筛子求素数法
pub fn solution() -> u64 {
  let max_number_to_check = 200_0000;
  let mut prime_mask = vec![true; max_number_to_check];
  prime_mask[0] = false;
  prime_mask[1] = false;

  let mut sum = 0_u64;
  for i in 2..max_number_to_check {
    if prime_mask[i] {
      sum += i as u64;

      let mut j = 2 * i;
      while j < max_number_to_check {
        prime_mask[j] = false;
        j += i;
      }
    }
  }
  sum
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 142913828922);
  }
}
