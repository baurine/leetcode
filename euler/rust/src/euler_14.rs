/*
https://projecteuler.net/problem=14

随便取一个数，如果这个数是奇数，那么它的下一个数是 3n+1，如果是偶数，而下一个数是 n/2，依此类推，直到这个数变成 1。

示例：13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

则 13 到 1 的序列长度为 10

求 1000000 以下的数里面，哪个数变成 1 的序列最长
*/
use std::collections::HashMap;

fn seq_len(mut n: u64) -> u32 {
  let mut len = 1;
  while n > 1 {
    // println!("==> {}", n);
    if n % 2 == 0 {
      n = n / 2;
    } else {
      n = 3 * n + 1;
    }
    len += 1;
  }
  len
}

fn seq_len_recursion(mut n: u64) -> u32 {
  if n == 1 {
    return 1;
  }
  if n % 2 == 0 {
    n = n / 2;
  } else {
    n = 3 * n + 1;
  }
  seq_len_recursion(n) + 1
}

fn seq_len_recursion_cached(n: u64, m: &mut HashMap<u64, u32>) -> u32 {
  if n == 1 {
    return 1;
  }

  if let Some(&len) = m.get(&n) {
    return len;
  }

  let next;
  if n % 2 == 0 {
    next = n / 2;
  } else {
    next = 3 * n + 1;
  }
  let len = seq_len_recursion_cached(next, m) + 1;
  m.insert(n, len);
  len
}

pub fn solution(solution_type: usize) -> (u32, u32) {
  let mut max_seq_start_num = 0;
  let mut max_seq_len = 0;

  let mut cache: HashMap<u64, u32> = HashMap::new();

  for i in 2..100_0000_u32 {
    // println!("==============================");
    // let cur_len = seq_len(i as u64);
    let cur_len = match solution_type {
      1 => seq_len(i as u64),
      2 => seq_len_recursion(i as u64),
      3 => seq_len_recursion_cached(i as u64, &mut cache),
      _ => 1,
    };
    if cur_len > max_seq_len {
      max_seq_len = cur_len;
      max_seq_start_num = i;
    }
  }
  // println!("{} -- {}", max_seq_start_num, max_seq_len);
  (max_seq_start_num, max_seq_len)
}

/////////////////////////////////////
// 注意 `#![feature(test)]` 要写在 main.rs 中，写在这里不行
extern crate test;

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn test_seq_len() {
    assert_eq!(seq_len(1), 1);
    assert_eq!(seq_len(2), 2);
    assert_eq!(seq_len(13), 10);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(1), (837799, 525));
    assert_eq!(solution(2), (837799, 525));
    assert_eq!(solution(3), (837799, 525));
  }

  #[bench]
  fn bench_solution_1(b: &mut Bencher) {
    b.iter(|| solution(1));
  }

  #[bench]
  fn bench_solution_2(b: &mut Bencher) {
    b.iter(|| solution(2));
  }

  #[bench]
  fn bench_solution_3(b: &mut Bencher) {
    b.iter(|| solution(3));
  }
}

// $ cargo bench euler_14
// benchmark 结果，和 Go 实现有些差别
// test euler_14::tests::bench_solution_1 ... bench: 165,094,604 ns/iter (+/- 5,242,748)
// test euler_14::tests::bench_solution_2 ... bench: 165,012,993 ns/iter (+/- 3,977,952)
// test euler_14::tests::bench_solution_3 ... bench: 345,677,224 ns/iter (+/- 79,896,263)
