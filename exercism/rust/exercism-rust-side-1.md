# Exercism Rust Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。每执行一次测试，exercism 就会记录一次当前的实现并作为历史记录一并提交到网站。

## Side exercises

### Leap Year

考查点：boolean, conditional

闰年计算，略。

### Raindrops

考查点：conditional, string

问题，将数字转换成字符串，按被 3/5/7 整除的情况转换成不同的字符串。

我的解法：

```rust
pub fn raindrops(n: u32) -> String {
  let mut s = String::default();
  if n % 3 == 0 {
    s.push_str("Pling");
  }
  if n % 5 == 0 {
    s.push_str("Plang");
  }
  if n % 7 == 0 {
    s.push_str("Plong");
  }
  if s.len() == 0 {
    s.push_str(&format!("{}", n));
  }
  s
}
```

然后看了一下社区的解法，有更简单的，也有用复杂的 (用 match 匹配了 9 种情况)，interesting! 而我的解法都是槽点。

初始化字符串可以用 `let mut s = String::new();`，字符串串接可以直接用 `+=`，String 支持这种操作，判空可以用 `s.is_empty()`，数字转换成字符串可以直接用 `n.to_string()`。

```rust
pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();
    if n % 3 == 0 {
        sound += "Pling";
    }
    if n % 5 == 0 {
        sound += "Plang";
    }
    if n % 7 == 0 {
        sound += "Plong";
    }
    if sound.is_empty() {
        sound = n.to_string();
    }
    sound
}
```

更具有扩展性的解法 (awesome)：

```rust
pub fn raindrops(n: u32) -> String {
  let sounds = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

  let s = sounds
    .iter()
    .filter(|item| n % item.0 == 0)
    .map(|item| item.1.to_string())
    .collect::<String>();

  if s.len() == 0 {
    n.to_string()
  } else {
    s
  }
}
```

还是要多向社区学习。

### Nth Prime

考查点：loop

问题：求第 n 个素数。

我的解法：

```rust
fn is_prime(num: u64) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false;
    }
  }
  true
}

pub fn nth(n: u32) -> u32 {
  let mut count = 0;
  for i in 2.. {
    if is_prime(i) {
      if count == n {
        return i as u32;
      }
      count += 1;
    }
  }
  0
}
```

mentor review，更希望看到使用迭代器的解决方案，参考社区后修改如下：

```rust
fn is_prime(num: u32) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false;
    }
  }
  true
}

struct Primes(u32);

impl Iterator for Primes {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    for i in (self.0 + 1).. {
      if is_prime(i) {
        self.0 = i;
        return Some(i);
      }
    }
    None
  }
}

pub fn nth(n: u32) -> u32 {
  Primes(1).nth(n as usize).unwrap()
}
```

看似简单的题目，也蕴含出题者背后的用心。

### Beer Song

这个之前用 ruby 练习过。但用 rust 练习了一遍后，看了一下社区的解决方法，发现我的仍然有很多可以改进的空间，尤其是用 match 替代 if...else if...else。sing() 方法用迭代器实现很精妙：`(end..=start).rev().map(verse).collect::<Vec<_>>().join("\n")`。注意，range 的范围只能是正向增长的，如果要反过来，必须用 rev() 方法。

我的原始 solution：

```rust
fn beverge(n: u32) -> String {
  if n > 1 {
    format!("{} bottles", n)
  } else if n == 1 {
    format!("{} bottle", n)
  } else {
    "No more bottles".to_string()
  }
}

fn remain_beverge(n: u32) -> String {
  if n >= 1 {
    beverge(n - 1)
  } else {
    beverge(99)
  }
}

fn action(n: u32) -> String {
  if n > 1 {
    "Take one down and pass it around".to_string()
  } else if n == 1 {
    "Take it down and pass it around".to_string()
  } else {
    "Go to the store and buy some more".to_string()
  }
}

pub fn verse(n: u32) -> String {
  format!(
    "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
    beverge(n),
    beverge(n).to_lowercase(),
    action(n),
    remain_beverge(n).to_lowercase()
  )
}

pub fn sing(start: u32, end: u32) -> String {
  let mut v: Vec<String> = vec![];
  for i in (end..=start).rev() {
    v.push(verse(i));
  }
  v.join("\n")
}
```

Refine 后：

```rust
fn beverge(n: u32) -> String {
  match n {
    0 => "No more bottles".to_string(),
    1 => "1 bottle".to_string(),
    _ => format!("{} bottles", n),
  }
}

fn remain_beverge(n: u32) -> String {
  match n {
    0 => beverge(99),
    _ => beverge(n - 1),
  }
}

fn action(n: u32) -> String {
  match n {
    0 => "Go to the store and buy some more",
    1 => "Take it down and pass it around",
    _ => "Take one down and pass it around",
  }
  .to_string()
}

pub fn verse(n: u32) -> String {
  format!(
    "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
    beverge(n),
    beverge(n).to_lowercase(),
    action(n),
    remain_beverge(n).to_lowercase()
  )
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start)
    .rev()
    .map(verse)
    .collect::<Vec<_>>()
    .join("\n")
}
```

### Proverb

考查点：format! 宏的使用。根据输入的单词数组，输出一首诗。

我的常规解法：

```rust
pub fn build_proverb(list: &[&str]) -> String {
  if list.len() == 0 {
    return "".to_string();
  }
  let mut v = vec![];
  for i in 0..list.len() - 1 {
    v.push(format!(
      "For want of a {} the {} was lost.",
      list[i],
      list[i + 1]
    ));
  }
  v.push(format!("And all for the want of a {}.", list[0]));
  v.join("\n")
}
```

但是社区里有很多有趣的解法。

解法 1：

```rust
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverbs = vec![];

    for (left, right) in list.iter().zip(list.iter().skip(1)) {
        proverbs.push(format!("For want of a {} the {} was lost.", left, right));
    }

    if let Some(first) = list.first() {
        proverbs.push(format!("And all for the want of a {}.", first));
    }

    proverbs.join("\n")
}
```

解法 2：

```rust
pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
        .chain(std::iter::once(list.first().map_or(String::new(), |v| {
            format!("And all for the want of a {}.", v)
        })))
        .collect()
}
```

### Difference Of Squares

考查点：fold, map, math

求 1 到 n 的和的平方，与平方的和，之间的差。

有点轻车熟驾了：

```rust
pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|i| i.pow(2)).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
```

### Sum of Multiples

考查点：borrowing, math, algorithms

问题：

Given a number, find the sum of all the unique multiples of particular numbers up to but not including that number.

If we list all the natural numbers below 20 that are multiples of 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.

The sum of these multiples is 78.

我的解法，一行代码搞定：

```rust
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|n| factors.iter().any(|f| f>&0 && n%f == 0)).sum::<u32>()
}
```

另一种解法，对 factors 进行迭代，每一个元素分别乘以 1 到 n/i，然后对所有结果进行去重，再求和。试一下。

```rust
use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&f| f > 0)
        .flat_map(|f| (1..(limit + f - 1) / f).map(move |i| i * f))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
```

对闭包的所有权的几种使用方式还是有点懵，需要再复习一下并多多练习。

### Grains

考查点：panic，处理错误

问题：就是农夫跟国王要粮食的故事，在棋盘上第一格放一粒米，第二格放二粒，后一格是前一格的二倍，求总和。

我的常规解法：

```rust
pub fn square(s: u32) -> u64 {
  if s > 64 || s < 1 {
    panic!("Square must be between 1 and 64");
  }
  2u64.pow(s - 1)
}

pub fn total() -> u64 {
  (1..=64).map(|n| square(n)).sum::<u64>()
}
```

社区解法，首先 square() 中的条件限制可以用 assert! 宏替代，其次 total() 可以直接用数学公式计算出来，就是 2^ 64 - 1，但直接这么算会溢出，所以用 2^63-1+2^63 替换。

```rust
pub fn square(s: u32) -> u64 {
  assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");
  2u64.pow(s - 1)
}

pub fn total() -> u64 {
  square(64) - 1 + square(64)
}
```

### Prime Factors

考查点：math

简单的数学计算，略。

### Armstrong Numbers

考查点：math

简单的数学计算，略。

### High Scores (v)

考查点：iterators, lifetimes, vectors

遇到一个有点挑战的题目。

问题：

Manage a game player's High Score list.

Your task is to build a high-score component of the classic Frogger game, one of the highest selling and addictive games of all time, and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.

Hints：

Consider retaining a reference to scores in the struct - copying is not necessary. You will require some lifetime annotations, though.

我的解法：

```rust
#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|s| *s)
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut v = self.scores.to_vec();
        v.sort();
        v.last().map(|s| *s)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort();
        v.iter().rev().take(3).map(|s| *s).collect::<Vec<_>>()
    }
}
```

对 Option 的处理，刚好看完 rust dao 第九章处理错误这一章，有相关的内容，比如 map() 方法，and_then() 方法。

考查到了生命周期，slice，vector，迭代器，自动解引用等 rust 的难点。虽然我最后做出来的通过了所有测试，但还是没有完全搞清楚，尤其是迭代器的闭包中到底是用值还是引用。

还需要重点复习的知识点：

- 自动解引用 (理解了)
- iter()，闭包的几种形式 (理解深一点了)
- 省略生命周期声明的几种情况 (待看)

经过 mentor review，一些可以改进的点：

- iterator 有 max() 方法
- sort() + iter() + rev() 方法可以用 sort_by() 替代
- take(3) 可以用 truncate(3) 替代

改进后：

```rust
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.to_vec().into_iter().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort_by(|a, b| b.cmp(a));
        v.truncate(3);
        v
    }
```
