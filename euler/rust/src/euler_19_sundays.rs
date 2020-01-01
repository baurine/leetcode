/*
https://projecteuler.net/problem=19

问题：求 20 世纪 (1901 年 1 月 1 日到 2000 年 12 月 31 日) 有多少个月的一号是星期日?

注意闰年，闰年 2 月有 29 天，其它时候是 28 天。

闰年：能被 4 和 400 整除，但不包括被 100 整除的年份。就是说一般情况下，闰年是每 4 年这一年就是闰年，2 月份会多攒出一天来。

但有特殊，当年份是 100 的倍数时候，虽然它也是 4 的倍数，但这个 100 不一定是闰年，这个 100 也要攒够 4 倍，即它要每 400 年才能攒出一个闰年来。

1900.1.1 是星期一
4/6/9/11 月 30 天
1/3/5/7/8/10/12 月 31 天
2 月 28/29 天

解题思路：计算每个月一号距离 1900.1.1 号有多少天，`%7` 可以算出它是星期几。
*/

fn leap_year(n: usize) -> bool {
  if n % 400 == 0 {
    true
  } else if n % 100 == 0 {
    false
  } else if n % 4 == 0 {
    true
  } else {
    false
  }
}

pub fn sundays() -> usize {
  let leap_year_months = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  let non_leap_year_months = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  let mut sum = 0;
  let mut acc_days = 0; // 从 1900.1.1 开始累积的天数总和
  for y in 1900..=2000 {
    for m in 1..=12 {
      let week_day = (acc_days + 1) % 7; // 0 是周日，1 是周一，2 是周二
      if week_day == 0 && y > 1900 {
        sum += 1;
      }
      acc_days += if leap_year(y) {
        leap_year_months[m]
      } else {
        non_leap_year_months[m]
      };
    }
  }
  sum
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_leap_year() {
    assert_eq!(leap_year(1900), false);
    assert_eq!(leap_year(2000), true);
    assert_eq!(leap_year(1904), true);
    assert_eq!(leap_year(2004), true);
    assert_eq!(leap_year(2100), false);
    assert_eq!(leap_year(2019), false);
    assert_eq!(leap_year(2020), true);
  }

  #[test]
  fn test_solution() {
    assert_eq!(sundays(), 171);
  }
}
