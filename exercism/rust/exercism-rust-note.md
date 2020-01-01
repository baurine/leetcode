# Exercism Rust Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。每执行一次测试，exercism 就会记录一次当前的实现并作为历史记录一并提交到网站。

## 1. Hello World

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

## 2. Reverse String

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

## 3. Giga Seconds

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
