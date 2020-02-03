# Exercism Go Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。

## Side exercise

### Leap year

考查点：boolean, conditional, int

闰年计算，略。

### Gigasecond

考查点：time pkg 使用

```go
package gigasecond

import "time"

const GIGASECONDS time.Duration = time.Second * 1_000_000_000

func AddGigasecond(t time.Time) time.Time {
	return t.Add(GIGASECONDS)
}
```

### Space Age

考查点：float

问题：将地球上的时间转换成其它星球上的时间

解决：略。主要是 float64 类型的使用

### Accumulate

考查点：lists

详略。

虽然简单，但看了一下社区的解决方案，还是玩出了一些骚操作，比如这样的：

```go
func Accumulate(src []string, converter func(string) string) (changed []string) {
	for _, s := range src {
		changed = append(changed, converter(s))
	}
	return
}
```

不过上面的解决方案代码是简洁了，但 slice 可以会多次被重新分配空间，效率不如一开始就分配好足够空间的高。

### Triangle

考查点：booleans, conditionals, logic

问题：判断三条边能否组成三角形，如果能组成，三角形的种类 (等边/对称)

解决：略。

刚好 rust 也做到 了 triangle 这一题，虽然是一样的题目，核心算法也一样，但 go 代码和 rust 代码的表现形式实在是迥异。
