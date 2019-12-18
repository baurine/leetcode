/*
https://projecteuler.net/problem=16

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?

解题思路：2^1000 已远超 2^64，所以无法用 u64 存储。可以用 Vec<u32> 存储。
1  -- 2
2  -- 4
3  -- 8
4  -- 6 1
5  -- 2 3
6  -- 4 6
7  -- 8 2 1
8  -- 6 5 2
9  -- 2 1 5
10 -- 4 2 0 1
*/

fn pow_sum(pow: u32) -> u32 {
  if pow == 1 {
    return 2;
  }
  let mut v = vec![2];
  let mut carry;
  for _i in 2..=pow {
    carry = 0;
    for item in &mut v {
      *item = (*item) * 2 + carry;
      carry = (*item) / 10;
      *item = (*item) % 10;
    }
    if carry > 0 {
      v.push(carry)
    }
  }
  v.iter().sum::<u32>()
}

pub fn solution() -> u32 {
  pow_sum(1000)
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pow_sum() {
    assert_eq!(pow_sum(1), 2);
    assert_eq!(pow_sum(2), 4);
    assert_eq!(pow_sum(3), 8);
    assert_eq!(pow_sum(4), 7);
    assert_eq!(pow_sum(5), 5);
    assert_eq!(pow_sum(6), 10);
    assert_eq!(pow_sum(10), 7);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 1366);
  }
}
