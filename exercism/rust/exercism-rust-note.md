# Exercism Rust Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。每执行一次测试，exercism 就会记录一次当前的实现并作为历史记录一并提交到网站。

## Tracks

### 1. Hello World

考查点：TDD，string

如何跑测试：

```sh
$ cargo test // 所有测试，但不包括被 `[ignore]` 声明的测试
$ cargo test -- --ignored // 跑所有被 `[ignore]` 声明的测试
$ cargo test some_test // 只跑某个具体的测试
$ cargo test some_test -- --ignored // 被 `[ignore]` 声明的某个具体的测试
```

格式化代码：

```sh
$ cargo fmt
```

检测无效代码：

```sh
$ cargo clippy --all-targets
```

### 2. Reverse String

考查点：string, iterator

反转字符串。常规解法：

```rust
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
```

但这种解法在遇到那种由多个 unicode 字素组成的单个字符时，会有问题，比如 `ü`，它实际是由一个两点的字素 `\u{308}` 加上 `u` 组成，rust 默认反转时是将字素反转，反转后就变成了 `u\u{308}`，但这样就无法还原成 `ü` 了。

因此，上面的解法无法通过这个测试：

```rust
#[test]
#[cfg(feature = "grapheme")]
/// grapheme clusters
fn test_grapheme_clusters() {
    process_reverse_case("uüu", "uüu");
}
```

测试方法：

```sh
$ cargo test --features grapheme
```

输出：

```
failures:

---- test_grapheme_clusters stdout ----
thread 'test_grapheme_clusters' panicked at 'assertion failed: `(left == right)`
  left: `"u\u{308}uu"`,
 right: `"uu\u{308}u"`', tests/reverse-string.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

crate unicode-segmentation 可以解决这个问题，它会按有意义的字符来逐个处理，而不是字素。

先添加依赖：

```toml
[dependencies]
unicode-segmentation = "1.6.0"

[features]
grapheme = []
```

实现：

```rust
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  input.graphemes(true).rev().collect()
}
```

### 3. Giga Seconds

考查点：crate

问题：求某个时间之后 10^9 秒后的时间。使用 chrono 这个 crate 完成需求。

比如一个测试是这样的：

```rust
use gigasecond;
use chrono::{TimeZone, Utc};

#[test]
fn test_date() {
    let start_date = Utc.ymd(2011, 4, 25).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2043, 1, 1).and_hms(1, 46, 40)
    );
}
```

实现很简单，chrono 这个 crate 的文档示例中正好有。

```rust
use chrono::{DateTime, Utc};
use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}
```

Duration 来自 time 这个 crate，chrono 依赖了 time crate，但上面的编译会报错，需要在 Cargo.toml 显式声明依赖 time 这个 crate。(疑惑，为什么还要显式声明，像 npm 都不用啊。==，npm 是真的不用吗？有待验证，即被依赖的库，我如果想直接在我的代码里使用，是否还需要在 package.json 中声明呢？)

```toml
[dependencies]
chrono = "0.4" # 这个原来就有的
time = "0.1" # 为了通过编译后加的
```

### 4. Clock

考查点：trait，derive, struct

solution:

```rust
use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const ONE_HOUR_MINS: i32 = 60;
const ONE_DAY_MINS: i32 = ONE_HOUR_MINS * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_mins = (hours * ONE_HOUR_MINS + minutes) % ONE_DAY_MINS;
        if total_mins < 0 {
            total_mins += ONE_DAY_MINS;
        }

        Clock {
            hours: total_mins / ONE_HOUR_MINS,
            minutes: total_mins % ONE_HOUR_MINS,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    // pub fn to_string(&self) -> String {
    //     format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

// impl PartialEq for Clock {
//     fn eq(&self, other: &Self) -> bool {
//         self.to_string() == other.to_string()
//     }
// }

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
```

学习到的：

1.  如果你为 struct 实现了 Display trait，那么它就自动拥了有 `to_string()` 方法。[原理](https://stackoverflow.com/a/27770058/2998877)：The `ToString` trait is already implemented for all types which implement `fmt::Display`。

    ```rust
    impl<T> ToString for T
    where
    T: Display + ?Sized,
    { /* ... */ }
    ```

1.  如果想让 struct 自动实现 PartialEq trait，那么这个 struct 必须先实现 Debug trait。因为 PartiqlEq 实现时是直接比较两个 struct 的 `{:?}` 实现的 String 的。

### 5. Atbash Cipher

对字符串进行加密和解密，加密解密算法是一样的，a->z，b->y 依此类推。

考查点：rust 如何处理字符串和字符。(刚好看完了 rust dao 第八章 - 字符串和集合类型，这里面就有相关的例子)

solution:

```rust
const A_LOWERCASE: u32 = 97;
const Z_LOWERCASE: u32 = A_LOWERCASE + 26 - 1;

fn convert_char(ori_char: char) -> char {
  if ori_char.is_numeric() {
    ori_char
  } else {
    std::char::from_u32(A_LOWERCASE + Z_LOWERCASE - ori_char as u32).unwrap()
  }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  plain
    .to_lowercase()
    .chars()
    .filter(|c| c.is_alphanumeric())
    .enumerate()
    .map(|(i, c)| {
      if (i + 1) % 5 == 0 {
        format!("{} ", convert_char(c))
      } else {
        convert_char(c).to_string()
      }
    })
    .collect::<String>()
    .trim()
    .to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  cipher
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| convert_char(c).to_string())
    .collect()
}
```

经过 mentor review，encode 和 decode 实际使用的是相同的算法，所以这部分还可以再抽出来。

```rust
fn convert_str(src: &str) -> Vec<char> {
  src
    .to_lowercase()
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(convert_char)
    .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  convert_str(plain)
    .iter()
    .enumerate()
    .map(|(i, c)| {
      if i % 5 == 0 && i > 0 {
        format!(" {}", c)
      } else {
        c.to_string()
      }
    })
    .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  convert_str(cipher).iter().collect()
}
```

但这还不够完美，因为 convert_str() 其实并不需要返回 String，直接返回迭代器就行了，但这种写法暂时还不知道该怎么实现。先跳过，回头再优化。

经过多达九轮提交和 mentor 的反复 review，最后代码实现如下：

```rust
use std::iter;

fn convert_char(ori_char: char) -> char {
  if ori_char.is_numeric() {
    ori_char
  } else if ori_char >= 'a' {
    // lowercase, 'a' = 97, convert to 'z'
    std::char::from_u32('a' as u32 + 'z' as u32 - ori_char as u32).unwrap()
  } else {
    // uppercase, 'A' = 65, convert to 'z'
    std::char::from_u32('A' as u32 + 'z' as u32 - ori_char as u32).unwrap()
  }
}

fn convert_str<'a>(src: &'a str) -> impl Iterator<Item = char> + '_ {
  src
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| convert_char(c))
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  convert_str(plain)
    .enumerate()
    .flat_map(|(i, c)| {
      iter::once(' ')
        .filter(move |_| i > 0 && i % 5 == 0)
        .chain(iter::once(c))
    })
    .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  convert_str(cipher).collect()
}
```

从中学习到了 `flat_map` `iter::once()` `chain()` 等的用法，返回迭代器的用法，但 `impl Iterator<Item = char> + '_` 中的 `'_` 生命周期标记还没太明白原理。待进一步学习。

把 `'_` 去掉后进行编译，提示说 `impl Iterator<Item = char>` 的生命周期是 static，但 `str.chars()` 是借用，提示加上一个匿名生命周期 `'_` 可以解决问题。

### 6. Anagram

考查点：lifetimes, loops, str vs strings, strings

问题：给定一个 word，从另一个字符串数组中，找出和这个 word 使用了相同字母但顺序不同的字符串。

实现：

```rust
use std::collections::HashSet;
use std::iter::FromIterator;

fn sort_lowercase_word(word: &str) -> String {
    let sorted_word = word.to_lowercase();
    let mut sorted_word = sorted_word.chars().collect::<Vec<char>>();
    sorted_word.sort();
    let sorted_word = sorted_word.iter().collect::<String>();
    sorted_word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_lowercase_word = sort_lowercase_word(word);

    let anagrams = possible_anagrams.iter().filter(|&item| {
        item.to_lowercase() != lowercase_word && sort_lowercase_word(item) == sorted_lowercase_word
    });

    HashSet::from_iter(anagrams.cloned())
}

//////

mod test {
    use super::*;

    #[test]
    fn test_sort_lowercase_word() {
        println!("{}", sort_lowercase_word("ΑΒΓ")); // αβγ ?? interesting
        assert_eq!(sort_lowercase_word("ΑΒΓ"), sort_lowercase_word("γβα"));
    }
}
```

对 lifetime 的理解加深了。

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

### Bob

考查点：chars 和 strings

问题：你朝 bob 说不同的话，它会进行不同的回复。

解答：

```rust
const REPLIES: [&'static str; 5] = [
  "Sure.",                             // question
  "Whoa, chill out!",                  // YELL
  "Calm down, I know what I'm doing!", // YELL and question, aka YELL?
  "Fine. Be that way!",                // empty
  "Whatever.",                         // others
];

pub fn reply(message: &str) -> &str {
  let msg = message.trim();
  let empty = msg.is_empty();
  let question = msg.chars().last() == Some('?');
  let alphas = msg
    .chars()
    .filter(|c| c.is_alphabetic())
    .collect::<String>();
  let yell = alphas.len() > 0 && alphas.chars().all(|c| c.is_uppercase());

  if empty {
    REPLIES[3]
  } else if yell && question {
    REPLIES[2]
  } else if yell {
    REPLIES[1]
  } else if question {
    REPLIES[0]
  } else {
    REPLIES[4]
  }
}
```

优化后：

```rust
const REPLY_QUESTION: &str = "Sure.";
const REPLY_YELL: &str = "Whoa, chill out!";
const REPLY_YELL_QUESTION: &str = "Calm down, I know what I'm doing!";
const REPLY_EMPTY: &str = "Fine. Be that way!";
const REPLY_OTHERS: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
  let msg = message.trim();

  let empty = msg.is_empty();
  let question = msg.ends_with('?');
  let yell = msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic());

  match (yell, question) {
    _ if empty => REPLY_EMPTY,
    (true, true) => REPLY_YELL_QUESTION,
    (true, _) => REPLY_YELL,
    (_, true) => REPLY_QUESTION,
    _ => REPLY_OTHERS,
  }
}
```

学习到了 `ends_with()`，判断全部为大写的简单办法：`msg.to_uppercase() == msg;`，用 match 替代 if...else。

### Matching Brackets

考查点：stack, recursion

问题：判断一个数学表达式括号是否匹配。

解答：

常规的栈的使用。暂时没想到用递归怎么解。看了一下社区解法也都是用栈实现的。我一开始写了个比较啰嗦的写法，用了 25 行，看了一下社区的解法，简洁写法可以达到 15 行，而且思路也巧妙一点。

```rust
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for c in string.chars() {
        match c {
            '(' => brackets.push(')'),
            '[' => brackets.push(']'),
            '{' => brackets.push('}'),
            ')' | ']' | '}' if brackets.pop() != Some(c) => {
                return false;
            }
            _ => (),
        }
    }
    brackets.is_empty()
}
```

上述的巧妙在于，遇到 "( [ {" 不是把它们自身放到栈里，而是把它们的另一半放到栈里。

### DOT DSL

开始进入 Medium 级别难度了。一来就写个 DSL?? 我...

考查点：builder pattern, derive, modules ...

问题：其实并不是要写 DSL，只是实现 Graph/Node/Edge 三种 struct 而已。

实现：

```rust
pub mod graph {

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    fn covert_attrs(attrs: &[(&str, &str)]) -> HashMap<String, String> {
        let mut new_attrs = HashMap::new();
        for item in attrs {
            new_attrs.insert(String::from(item.0), String::from(item.1));
        }
        new_attrs
    }

    pub mod graph_items {
        use std::collections::HashMap;

        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: super::HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: super::HashMap::new(),
                    }
                }

                // pub fn with_attrs<'a>(&'a mut self, attrs: &[(&str, &str)]) -> &'a mut Self {
                //     for item in attrs {
                //         self.attrs
                //             .insert(String::from(item.0), String::from(item.1));
                //     }
                //     self
                // }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: crate::graph::covert_attrs(attrs),
                        // must put it in the end
                        ..self
                    }
                }

                pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
                    // self.attrs.get(attr_key).map(|x| (*x).as_str())
                    // self.attrs.get(attr_key).map(|x| x.as_str())
                    // self.attrs.get(attr_key).map(|x| (*x).as_ref())
                    self.attrs.get(attr_key).map(|x| x.as_ref())
                }
            }
        }

        pub mod edge {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                node_start: String,
                node_end: String,
                attrs: super::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node_start: &str, node_end: &str) -> Self {
                    Edge {
                        node_start: String::from(node_start),
                        node_end: String::from(node_end),
                        attrs: super::HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: crate::graph::covert_attrs(attrs),
                        ..self
                    }
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.clone(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Graph {
                edges: edges.clone(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: crate::graph::covert_attrs(attrs),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&item| item.name == name)
        }
    }
}
```

学习到的：

- mod 的组织
- Vec 的 clone
- Rust 中的 spread 操作是两个点，且只能放到最后，不能放在别的位置
- `get_attr()` 方法的实现，有待进一步学习。&String 可以自动解引用为 &str，但 `Option<&String>` 并不能自动解引用为 `Option<&str>`，String 转成 &str 可以用 `as_str()` 或 `as_ref()`，再学习一下类型转换
- HashMap
- iterator find()
