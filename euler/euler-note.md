# Euler Note

对 euler 解题的一些总结。加 v 的表示有点意思，可用于面试。

递归被重度使用。

## 第 1 ~ 12 题

略。基本都是暴力枚举。

## 第 13 题 (v)

问题：

100 个 50 位数的和的最高 10 位数。

解题思路：

将每个 50 位拆成 5 个 10 位数，先将 100 个最后 10 位数相加，得到进位数，然后将次后 10 位数相加，加上前面得到的进位数，
依次这样操作。

## 第 14 题

问题：

随便取一个数，如果这个数是奇数，那么它的下一个数是 3n+1，如果是偶数，而下一个数是 n/2，依此类推，直到这个数变成 1

示例：13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

则 13 到 1 的序列长度为 10

求 1000000 以下的数里面，哪个数变成 1 的序列最长

解题思路：暴力枚举 (试了缓存中间结果，但缓存命中率太低，效率反而低)

## 第 15 题 (v)

问题：一个 2x2 的网格，总共有 6 种路径可以从左上角到右下角，求 20x20 的网格有多少种路径。

解题思路：递归 + 缓存中间结果 (不缓存中间结果的话需要很长时间才能得到结果)

## 第 16 题 (v)

问题：求 2^1000 的十进制的值的每位数加起来总和。

解题思路：用 `Vec<u32>` 存储

```
n  -- vec
1  -- 2
2  -- 4
3  -- 8
4  -- 6 1
5  -- 2 3
6  -- 4 6
7  -- 8 2 1
8  -- 6 5 2
9  -- 2 1 5
10 -- 4 2 0 1
```

收获：

go 里面使用 append 往切片里添加元素：`slice = append(slice, new_item)`

## 第 17 题

问题：求 1 到 1000 这 1000 个数转换成英语单词后的长度总和。

解题思路：分解，递归。

收获：

rust 解法中学习到了 match 和 match guard, format!() 的用法。

go 中用 fmt.Sprintf() 进行字符串格式化，用 `strings.split(str, spliter)` 进行 split。

## 第 18 题 (v)

问题：求一个大三角 (15 行) 从顶端到底部的一条路径，使路径上的值的和为最大值，求这个最大值。

```
   3
  7 4
 2 4 6
8 5 9 3
```

比如上例中，和最大值路径为 2 -> 7 -> 4 -> 9，和为 23。

解题思路：可以转换成求二叉树的和最大值路径，依旧使用递归法。

收获：Go 里面没有三目表达式。

## 第 19 题

问题：已知 1900.1.1 是周一，求 20 世纪 (1901 年 1 月 1 日到 2000 年 12 月 31 日) 有多少个月的一号是星期日?

注意闰年，闰年 2 月有 29 天，其它时候是 28 天。

闰年：能被 4 和 400 整除，但不包括被 100 整除的年份。就是说一般情况下，每 4 年能攒出一个闰年，2 月份会多攒出一天来。

但有特殊，当年份是 100 的倍数时候，虽然它也是 4 的倍数，但这个 100 不一定是闰年，这个 100 也要攒够 4 倍，即它要每 400 年才能攒出一个闰年来。

解题思路：

常规解法，先计算出每个月一号距离 1900.1.1 有多少天，然后用这个数 %7，再根据 1900.1.1 是周一就可以算出这个月一号是星期几。

其它解法，Rust 直接用 chrono 这个处理日期时间的 crate，可以直接计算某一天是星期几。Go 可以用自带的 time 这个 pkg。

## 第 20 题

问题：求 100! (=100x99x98...1) 的值每一位相加后的和。

解题思路：和第 16 题一样，因为 100 的阶乘太大了，即使用 64 位也无法存储下，所以用数组存储结果的每一位。和第 16 题稍微不一样的地方在于，最后的进位可以大于 10，所以不能像第 16 题那样直接把 carry push 到数组中，而是要对 carry 进行分解成多位分别 push 到数组中。

```rust
    // 和第 16 题不一样的地方
    // 这里的 carry 可以大于 10
    // if carry > 0 {
    //   fac_vec.push(carry);
    // }
    while carry > 0 {
      fac_vec.push(carry % 10);
      carry = carry / 10;
    }
```

## 第 21 题 - Amicable numbers

问题：

Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

解题思路：

没啥捷径，暴力枚举。虽然简单，为了通过测试也费了点时间，主要是一些边缘 case，比如 d(a) = b，结果 a = b 的情况没有排除掉。

学习了 rust/go 中的数学模块，求 sqrt/floor，sort，int/float 互转。

rust 中求 sqrt/floor: `(n as f64).sqrt().floor() as usize`。

go 中求 sqrt/floor，需要 `import "math"`，`int(math.Floor(math.Sqrt(float64(n))))`。

rust 中 sort: `v.sort();`

go 中 sort，需要 `import "sort"`，`sort.Ints(v)`。

(暂时中断 euler 练习，优先完成 exercism 上的练习)
