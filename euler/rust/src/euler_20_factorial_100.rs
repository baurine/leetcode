/*
https://projecteuler.net/problem=12

问题：求 100 阶乘后的数每一位相加后的总和

解题思路：跟 16 题一个思路，用 vec 存储结果
*/

fn factorial_n_bits_vec(n: usize) -> Vec<usize> {
  let mut fac_vec = vec![1];
  let mut carry;
  for i in 1..=n {
    carry = 0;
    for item in &mut fac_vec {
      *item = (*item) * i + carry;
      carry = (*item) / 10;
      *item = (*item) % 10;
    }
    // 和第 16 题不一样的地方
    // 这里的 carry 可以大于 10
    // if carry > 0 {
    //   fac_vec.push(carry);
    // }
    while carry > 0 {
      fac_vec.push(carry % 10);
      carry = carry / 10;
    }
  }
  fac_vec
}

fn factorial_n_bits_sum(n: usize) -> usize {
  factorial_n_bits_vec(n).iter().sum::<usize>()
}

pub fn factorial_100_bits_sum() -> usize {
  factorial_n_bits_sum(100)
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factorial_n_bits_vec() {
    assert_eq!(factorial_n_bits_vec(1), vec![1]);
    assert_eq!(factorial_n_bits_vec(2), vec![2]);
    assert_eq!(factorial_n_bits_vec(3), vec![6]);
    assert_eq!(factorial_n_bits_vec(4), vec![4, 2]);
    assert_eq!(factorial_n_bits_vec(5), vec![0, 2, 1]);
    assert_eq!(factorial_n_bits_vec(6), vec![0, 2, 7]);
    assert_eq!(factorial_n_bits_vec(10), vec![0, 0, 8, 8, 2, 6, 3]);
  }

  #[test]
  fn test_factorial_n_bits_sum() {
    assert_eq!(factorial_n_bits_sum(6), 9);
    assert_eq!(factorial_n_bits_sum(10), 27);
  }

  #[test]
  fn test_factorial_100_bits_sum() {
    // println!("{:?}", factorial_n_bits_vec(100));
    assert_eq!(factorial_100_bits_sum(), 648);
  }
}
