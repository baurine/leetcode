/*
https://projecteuler.net/problem=14

随便取一个数，如果这个数是奇数，那么它的下一个数是 3n+1，如果是偶数，而下一个数是 n/2，依此类推，直到这个数变成 1。

示例：13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

则 13 到 1 的序列长度为 10

求 1000000 万以下的数里面，哪个数变成 1 的序列最长
*/

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

pub fn solution() -> (u32, u32) {
  let mut max_seq_start_num = 0;
  let mut max_seq_len = 0;
  for i in 2..100_0000_u32 {
    // println!("==============================");
    let cur_len = seq_len(i as u64);
    if cur_len > max_seq_len {
      max_seq_len = cur_len;
      max_seq_start_num = i;
    }
  }
  // println!("{} -- {}", max_seq_start_num, max_seq_len);
  (max_seq_start_num, max_seq_len)
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_seq_len() {
    assert_eq!(seq_len(1), 1);
    assert_eq!(seq_len(2), 2);
    assert_eq!(seq_len(13), 10);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(), (837799, 525));
  }
}
