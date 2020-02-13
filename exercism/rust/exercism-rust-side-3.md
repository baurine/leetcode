# Exercism Rust Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。每执行一次测试，exercism 就会记录一次当前的实现并作为历史记录一并提交到网站。

## Side exercises

### Grade School

考查点：entry api, option, structs

实际使用了 BTreeMap/BTreeSet 解决，免去了自动手动排序，看了社区，很多人使用 HashMap 和 Vec，需要自己排序。

### Binary Search

考查点：slices, option type, traits

问题：对数组进行二分查找

解决：迭代法和递归法。

学习到的：

- slice 的更多用法：
  - 直接用 index 取值：`slice[mid]`
  - `slice[..mid]`
  - `slice[mid+1..]`
  - `let (left, right) = slice.split_at(middle)`
- AsRef trait 的作用和使用
  - [AsRef 和 AsMut](https://wiki.jikexueyuan.com/project/rust-primer/intoborrow/asref.html)

### Robot Simulator

考查点：immutability, enums

问题：给定一个机器人初始坐位及一串指定 (左转/右转/向前)，得到最终坐标。

解决：略。

学习到的：

- fold() 方法的使用：`instructions.chars().fold(self, |acc, c| ...)`
- `(self) -> Self` 的方法，如果 self 没有实现 Copy，则会发生所有权转移，实际只要参数是 self，不管返回值是啥，都会发生所有权转移，所有权转移到方法内部然后就被消耗掉了，之后原来这个对象就不能使用了，所以这种方法只能调用一次。

(对闭包又有点蒙了，如果对闭包使用 move 操作，且捕获变量是引用型的，则会发生所有权转移，但闭包却并不一定会消耗这个变量，所以闭包可以多次调用，是 Fn 或 FnMut 类型，而不是 FnOnce，和这里只能调用一次不一样啊，再研究研究。)

对于闭包而言，通过 move 得到所有权并不等于 T 捕获。

看这个例子：

```rust
fn main() {
  let b = Box::new(12);
  let f = move || {
      println!("b = {}", b);
  };
  f();
  f();
  // println!("b = {}", b); // error
}
```

f 得到了 b 的所有权，但里面的代码只是对 b 的引用，所以 f 还是 Fn 类型，可以被调用多次。

假如如下修改代码：

```rust
fn main() {
  let b = Box::new(12);
  let f =  || {
      println!("b = {}", b);
      b;
  };
  f();
  // println!("b = {}", b); // error
  // f(); // error FnOnce
}
```

在 f 中直接以 T 的形式使用 b 且离开作用域时 b 会被析构掉，所以 f 只能被调用一次，f 是 FnOnce。

### Queen Attack

考查点：result type, structs, traits

问题：判断国际象棋棋盘上两个王后能否相互攻击。

解决：判断两个坐标是否在同一水平或对角线上。

学习到的：

- 求绝对值：`(x-y).abs()`
- match 中使用 range:

```rust
match (rank, file) {
  (0..8, 0..8) => Some(...),
  _ => None
}
```

### Bowling

考查点：goofy bowling logic, result type

问题：根据保龄球的计分规则，得到一场保龄球的最后得分。

解决：略。(看了社区方案，也是各种各样，看到一个最少的是 50 行，我的有 110 行，大部分的都是 100 行左右)

学习到的：

- bowling 的计分规则，还真是有点小复杂，分失误/补中/全中三种情况。https://www.jianshu.com/p/4c737ce5baac
- 又遇到了 mutable borrow 和 borrow 同时使用导致编译不通过的情况，没想到什么好的解决办法，用了 workround 绕过去了

### Tournament

考查点：sorting, structs, enums, hashmaps (我没有用到 enums)

问题：根据各足球队之间的比赛结果生成最终的计分表。

解决：略。

逐渐有点得心应手了。

忘了直接用 `format!()` 也可以得到 String，但我觉得为 Team 实现 fmt::Display trait 更佳。

看到一个不错的社区方案 (https://exercism.io/tracks/rust/exercises/tournament/solutions/2d4c31fe40c9459193cb8313f94c3f56)：

```rust
//...
        scores
            .entry(y[0])
            .and_modify(|stats| *stats = *stats + team1result)
            .or_insert(team1result);

        scores
            .entry(y[1])
            .and_modify(|stats| *stats = *stats + team2result)
            .or_insert(team2result);

    let mut vecscores: Vec<(&str, Stats)> = scores.drain().collect();
    vecscores.sort_by(
        |(key1, value1), (key2, value2)| match value1.Score().cmp(&value2.Score()) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            _ => match key1.partial_cmp(key2) {
                Some(Order) => Order,
                None => Ordering::Equal,
            },
        },
    );
//...
```

`entry().and_modify().or_insert()`, `drain()` 学习一下。

### Alphametics

考查点：combinations, parsing, strings

问题：解迷题，比如 "SEND + MORE = MONEY"，每个字母表示 0~9 中的一位数字，首位不能是 0，求各字母代表的数字。

解决：略。看代码。

`(0..10).permutations(n)` 帮大忙了，将 0~9 进行排列组合。否则就要从 10^(n-1) 到 10^n 进行暴力枚举，当 n = 10 时要跑很久很久。

permutations:

- [分享一个由 Raymond Hettinger 编写的字母算术解迷函数](https://pantao.parcmg.com/press/solve-cryptarithms-by-raymond-hettinger-via-python.html)
