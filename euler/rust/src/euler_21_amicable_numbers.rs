/*
https://projecteuler.net/problem=21

问题：

Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

解题思路：暴力枚举吧
*/

fn divisors(n: usize) -> Vec<usize> {
  let mut v = Vec::new();
  for i in 1..=(n as f64).sqrt().floor() as usize {
    if n % i == 0 {
      v.push(i);

      let divisor = n / i;
      if divisor > i && divisor < n {
        v.push(divisor);
      }
    }
  }
  v.sort();
  v
}

fn divisors_sum(n: usize) -> usize {
  divisors(n).iter().sum()
}

fn amicable_numbers_sum() -> usize {
  let mut v = Vec::new();
  for i in 1..10000 {
    if v.contains(&i) {
      continue;
    }
    let sum = divisors_sum(i);
    if sum == i {
      continue;
    }
    if sum < 10000 && divisors_sum(sum) == i {
      if !v.contains(&i) {
        v.push(i);
      }
      if !v.contains(&sum) {
        v.push(sum);
      }
    }
  }
  // println!("{:?}", v);
  v.iter().sum()
}

//////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_divisors() {
    assert_eq!(divisors(1), vec![1]);
    assert_eq!(divisors(2), vec![1]);
    assert_eq!(divisors(3), vec![1]);
    assert_eq!(divisors(4), vec![1, 2]);
    assert_eq!(divisors(5), vec![1]);
    assert_eq!(divisors(6), vec![1, 2, 3]);
    assert_eq!(divisors(220), vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]);
    assert_eq!(divisors(284), vec![1, 2, 4, 71, 142]);
  }

  #[test]
  fn test_divisors_sum() {
    assert_eq!(divisors_sum(1), 1);
    assert_eq!(divisors_sum(2), 1);
    assert_eq!(divisors_sum(3), 1);
    assert_eq!(divisors_sum(4), 3);
    assert_eq!(divisors_sum(5), 1);
    assert_eq!(divisors_sum(6), 6);
    assert_eq!(divisors_sum(220), 284);
    assert_eq!(divisors_sum(284), 220);
  }

  #[test]
  fn test_amicable_numbers_sum() {
    assert_eq!(amicable_numbers_sum(), 1000);
  }
}
