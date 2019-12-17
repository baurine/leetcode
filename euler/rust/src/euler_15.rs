/*
https://projecteuler.net/problem=15

一个 2x2 的网格，总共有 6 种路径可以从左上角到右下角，求 20x20 的网格有多少种路径。

解题思路：递归法。p(m, n) = p(m-1, n) + p(m-1, n-1) + p(m-1, n-2) + ... p(m-1, 0)

但纯递归法耗时太多，可以把中间结果缓存下来。(嗯，可以回头看看第 14 题了，也可以用递归法加缓存中间结果)

?? Rust 怎么跑 benchmark
*/

// 递归法，只能用于 m 和 n 较小时
fn path_slow(m: u32, n: u32) -> u32 {
  if m == 0 || n == 0 {
    return 1;
  }
  let mut sum = 0;
  for i in 0..=n {
    sum += path_slow(m - 1, i);
  }
  return sum;
}

// 将中间的计算结果缓存下来，m 和 n 最大值不能超过 100
fn path_slow_2(v: &mut Vec<u64>, m: usize, n: usize) -> u64 {
  if m == 0 || n == 0 {
    return 1;
  }
  if v[m * 100 + n] > 0 {
    return v[m * 100 + n];
  }
  let mut sum = 0u64;
  for i in 0..=n {
    sum += path_slow_2(v, m - 1, i);
  }
  v[m * 100 + n] = sum;
  return sum;
}

pub fn solution() -> u32 {
  path_slow(12, 12)
}

pub fn solution_2() -> u64 {
  let mut v: Vec<u64> = vec![0; 100 * 100];
  path_slow_2(&mut v, 20, 20)
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_path_slow() {
    assert_eq!(path_slow(1, 1), 2);
    assert_eq!(path_slow(2, 2), 6);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 2704156);
  }

  #[test]
  fn test_solution_2() {
    assert_eq!(solution_2(), 137846528820);
  }
}
