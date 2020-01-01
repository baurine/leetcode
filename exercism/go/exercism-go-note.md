# Exercism Go Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。

## 1. Hello World

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

## 2. two-fer

```go
package twofer

import "fmt"

func ShareWith(name string) string {
	shartTarget := name
	if len(name) == 0 {
		shartTarget = "you"
	}
	return fmt.Sprintf("One for %s, one for me.", shartTarget)
}
```

Go 中使用 `fmt.Sprintf()` 格式化字符串，相当于 js 中的字符串插值的用法。

## leap year

闰年计算，略。
