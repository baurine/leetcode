/*
https://projecteuler.net/problem=17

If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
*/

// 递归
// 学习到了 match，match guard，format! 用法
fn number_to_words(num: usize) -> String {
  match num {
    1..=9 => {
      let words = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
      ];
      format!("{}", words[num])
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
      format!("{}", words[num % 10])
    }
    20..=99 if num % 10 == 0 => {
      let words = vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
      ];
      format!("{}", words[num / 10])
    }
    20..=99 => {
      // 35 拆成 30 + 5
      format!(
        "{} {}",
        number_to_words(num - num % 10),
        number_to_words(num % 10)
      )
    }
    100..=999 if num % 100 == 0 => format!("{} hundred", number_to_words(num / 100)),
    100..=999 => {
      // 543 -> 5 hundred and 43
      format!(
        "{} hundred and {}",
        number_to_words(num / 100),
        number_to_words(num % 100)
      )
    }
    1000 => format!("{}", "one thousand"),
    _ => format!("{}", ""),
  }
}

fn number_words_len(num: usize) -> usize {
  let words = number_to_words(num);
  // words.split(" ").collect::<Vec<&str>>().iter().map(|w| w.len()).sum::<usize>()
  words.split(" ").map(|w| w.len()).sum::<usize>()
}

pub fn solution() -> usize {
  (1..=1000).map(|n| number_words_len(n)).sum::<usize>()
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_number_words() {
    assert_eq!(number_to_words(1), "one".to_string());
    assert_eq!(number_to_words(2), "two".to_string());
    assert_eq!(number_to_words(12), "twelve".to_string());
    assert_eq!(number_to_words(50), "fifty".to_string());
    assert_eq!(number_to_words(54), "fifty four".to_string());
    assert_eq!(number_to_words(600), "six hundred".to_string());
    assert_eq!(
      number_to_words(654),
      "six hundred and fifty four".to_string()
    );
  }

  #[test]
  fn test_number_word_len() {
    assert_eq!(number_words_len(1), 3);
    assert_eq!(number_words_len(2), 3);
    assert_eq!(number_words_len(12), 6);
    assert_eq!(number_words_len(50), 5);
    assert_eq!(number_words_len(54), 9);
    assert_eq!(number_words_len(600), 10);
    assert_eq!(number_words_len(654), 22);
  }

  #[test]
  fn test_solution() {
    assert_eq!(solution(), 21124);
  }
}
