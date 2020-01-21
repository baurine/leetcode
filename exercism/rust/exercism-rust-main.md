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
