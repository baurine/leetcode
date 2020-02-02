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
