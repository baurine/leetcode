# Exercism Go Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。

## Tracks

### 1. Hello World

考查点：TDD, string

如何跑测试：

```sh
$ go test
$ go test -run <regexp> // 只跑符合正则表达式的测试
```

如何跑 benchmark:

```sh
$ go test -v --bench . --benchmem
```

格式化代码：

```sh
$ go fmt
$ go vet
```

### 2. two-fer

考查点：conditional, string

```go
package twofer

import "fmt"

func ShareWith(name string) string {
	shareTarget := name
	if len(name) == 0 {
		shareTarget = "you"
	}
	return fmt.Sprintf("One for %s, one for me.", shareTarget)
}
```

Go 中使用 `fmt.Sprintf()` 格式化字符串，相当于 js 中的字符串插值的用法。

经过 mentor review 和看了社区的解决方法后修改如下：

```go
package twofer

import "fmt"

func ShareWith(name string) string {
	if len(name) == 0 {
		name = "you"
	}
	return "One for " + name + ", one for me."
}
```

Go 也支持用 `+` 进行字符串拼接。

### 3. Hamming

汉明码，求两个相同长度字符串进行比较，不同的字符个数。

考查点：比较，loop，strings

```go
package hamming

import "errors"

// Distance of 2 same length DNA sequences
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("lens is different")
	}
	diffCount := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			diffCount++
		}
	}
	return diffCount, nil
}
```

我学习到的：

- error 实例的构造：`errors.New(...)`
- 导出函数最好有注释，且注释要以函数名开头

### 4. Raindrops

将数字在能被 3/5/7 整除的情况下转换成不同的字符串，如果都不能整除，则返回自身。在 Rust 的 exercism 中练习过了。

```go
package raindrops

import "strconv"

// Convert number to raindrop sound
func Convert(n int) string {
	sound := ""
	if n%3 == 0 {
		sound += "Pling"
	}
	if n%5 == 0 {
		sound += "Plang"
	}
	if n%7 == 0 {
		sound += "Plong"
	}
	if sound == "" {
		sound = strconv.Itoa(n)
	}
	return sound
}
```

Rust 中数字转换成字符串可以用 `n.to_string()`，Go 不行，可以用 `strconv.Itoa(n)`。

Rust 中字符串转换在数字可以用 `"42".parse()`，Go 还是用 strconv 这个包的方法 `strconv.Atoi("42")`。

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
