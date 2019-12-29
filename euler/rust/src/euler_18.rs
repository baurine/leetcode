/*
https://projecteuler.net/problem=18

问题：三角，求一条路径，使其从最顶端到最底端和最大。

解题思路：(有点跟求二叉树最大值路径相似，或者相同?)

依旧是递归法。
*/

fn row(n: usize) -> usize {
  let mut s = 0;
  for row in 1.. {
    s += row;
    if s > n {
      return row;
    }
  }
  0
}

fn max_path_weight(w: &[u32], n: usize) -> u32 {
  if n >= w.len() {
    return 0;
  }
  let cur_row = row(n);
  let last_row = row(w.len() - 1);
  if cur_row == last_row {
    return w[n]; // 退出条件，如果 n 已经是处于最后一行了
  }
  let left = max_path_weight(w, n + cur_row); // 当前节点下一行左边节点的位置为 n + cur_row
  let right = max_path_weight(w, n + cur_row + 1); // 当前节点下一行右边节点的位置为 n + cur_row + 1
  let max = if left > right { left } else { right };
  w[n] + max // 返回当前值和下一轮中的更大值之和
}

pub fn solution() -> u32 {
  let w = [
    75, // row 1
    95, 64, //row2
    17, 47, 82, //row3
    18, 35, 87, 10, // row4
    20, 04, 82, 47, 65, // row5
    19, 01, 23, 75, 03, 34, // row6
    88, 02, 77, 73, 07, 63, 67, // row7
    99, 65, 04, 28, 06, 16, 70, 92, // row8
    41, 41, 26, 56, 83, 40, 80, 70, 33, // row9
    41, 48, 72, 33, 47, 32, 37, 16, 94, 29, // row10
    53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, // row11
    70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, // row12
    91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, // row13
    63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, // row14
    04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23, // row15
  ];
  max_path_weight(&w, 0)
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_row() {
    assert_eq!(row(0), 1);
    assert_eq!(row(1), 2);
    assert_eq!(row(2), 2);
    assert_eq!(row(17), 6);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 1074);
  }
}
