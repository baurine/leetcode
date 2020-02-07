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
