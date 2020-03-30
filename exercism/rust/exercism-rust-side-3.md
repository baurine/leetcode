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

### Two Bucket

Refs:

- [逻辑思维编程 - 倒水问题](https://blog.csdn.net/huyoufu200920201078/article/details/78989161)
- [倒水问题的万能解法（扩展欧几里德算法）](https://blog.csdn.net/lanchunhui/article/details/50594649)
- [倒水问题 - 经典面试题](https://www.iteye.com/blog/gaotong1991-2043689)

解法：扩展欧几里德算法。

问题可以归纳成：两个 x 升和 y 升的桶，能否得到 z 升的容量，z <= max(x,y)。

首先要判断能否实现。对 x 乘以 1..n，然后对 y 取模，如果有一个余数等于 z，则可以得到，如果没有任何余数等于 z，则不能得到。

具体的步骤：

1. 先将 x 升的桶 1 填满
1. 将桶 1 倒入桶 2，这里有两种情况
   1. 如果桶 2 的剩余空间大于桶 1 的当前容量，如此操作后，桶 1 为 0，桶 2 为原来的容量加上桶 1 的容量
   1. 如果桶 2 的剩余空间小于等于桶 1 的当前容量，如此操作后，桶 2 补倒满，桶 1 剩余一些容量。将被倒满的桶 2 清空
1. 对比桶 1 或桶 2 是否达到目标容量
1. 依次重复上面的步骤，直到实现目标容量

大致实现就是这样。

但这一题加了一些额外的内容，增加了一点点复杂度。上面的解法都是从填充桶 1 开始的，但这里允许从桶 2 开始填充，而且不允许把桶 2 填充满后再倒掉，再填满桶 1，从而转换成从桶 1 开始填充的情况。

但实际确实可以转换，而且更简单，那就是直接把桶 2 当成桶 1，把桶 1 当成桶 2。即 `solve(bucket_1, bucket_2, goal, &Bucket::One) == solve(bucket_2, bucket_1, goal, &Bucket::Two)`。

### Spiral Matrix

略。需要一个方向变量来控制当前移动的方向。在试探下一步时用一个 `next_pos` 来临时存储下一步，然后判断 `next_pos` 是不是有效，如果有效，则用这个 `next_pos`，否则，变更方向，再产生新的下一步。

可以用 `vec![0; size as usize];` 初始化一个 vector 并全部填充为相同的值。

### Palindrome Products

问题：求一个区间内 [m, n] 的任意两数相乘的所有数字中，找到是回文的数字中的最大值，与最小值，分别返回它们的因数，比如 9 它的因数是 `[(1,9), (3,3)]`。

没什么好办法，暴力枚举，稍微改进一下的话，就是加一个缓存，存储已经判断是否为回文的的数字，因为判断回文是操作字符串，应该是计算瓶颈，但即便如此，当 m 和 n 是 4 位数时，处理时间依然超过了 60s。

判断回文的逻辑：

```rust
fn is_palindrome(n: u64) -> bool {
    n.to_string().chars().rev().collect::<String>() == n.to_string()
}
```

这个例子重度使用 match，几科每一步都用 match 来进行判断。对 match 时所用权的转移还有待复习，加深理解。

### Saddle Points

问题：求一个矩阵中，在行上为最大值，列上为最小值的那些值。

解决：暴力枚举。

用到的迭代器方法：enumerate(), all()

### Isogram

考查点：chars, iterators, strings

问题：判断一个字符串里有没有重复的字符 (不包括空格和连字符)，如果没有重复则为 isogram。

解决：遍历，使用 HashSet 存储遍历过的字符。

瞄了一眼社区的解决方法，看了前几个，解法是千奇百怪，没一个人用这我种朴素的方法，都是迭代器一迭到底，比如 `candidate.chars().filter().map().all()`，大部分都用了 all() 这个方法。

贴个[例子](https://exercism.io/tracks/rust/exercises/isogram/solutions/7fdbc2e4b59345008de6eb7679bbf38a)：

```rust
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut chr_set = HashSet::new();
    candidate.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| chr_set.insert(c))
}
```

仔细研究了一下，倒确实是很巧妙啊，HashSet 的 insert() 方法返回的是 bool 值，如果值已存在，则表示插入失败，返回 false，否则返回 true。

rust 和 go 相比，奇淫技巧很是多啊。相同的问题，rust 的解法每个人都不同，但用 go 的话，每个人的都大同小异。
