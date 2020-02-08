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
