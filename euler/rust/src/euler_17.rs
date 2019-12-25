/*
https://projecteuler.net/problem=17

If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
*/

// 递归
// 学习到了 match 和 match guard 用法
fn number_word_len(num: usize) -> usize {
  match num {
    1..=9 => {
      let words = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
      ];
      words[num].len()
    }
    10..=19 => {
      let words = vec![
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
      ];
      words[num % 10].len()
    }
    20..=99 if num % 10 == 0 => {
      let words = vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
      ];
      words[num / 10].len()
    }
    20..=99 => {
      // 35 拆成 30 + 5
      number_word_len(num - num % 10) + number_word_len(num % 10)
    }
    100..=999 if num % 100 == 0 => number_word_len(num / 100) + "hundred".len(),
    100..=999 => {
      // 543 -> 5 hundred and 43
      number_word_len(num / 100) + "hundred".len() + "and".len() + number_word_len(num % 100)
    }
    1000 => "one".len() + "thousand".len(),
    _ => 0,
  }
}

pub fn solution() -> usize {
  (1..=1000).map(|n| number_word_len(n)).sum::<usize>()
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_number_word_len() {
    assert_eq!(number_word_len(1), 3);
    assert_eq!(number_word_len(2), 3);
    assert_eq!(number_word_len(12), 6);
    assert_eq!(number_word_len(50), 5);
    assert_eq!(number_word_len(54), 9);
    assert_eq!(number_word_len(600), 10);
    assert_eq!(number_word_len(654), 22);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 21124);
  }
}
