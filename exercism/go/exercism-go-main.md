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

### 5. Scrabble Score

考查点：loops, maps, strings

问题：每个字符有一个权重值，求一个字符串的总权重值。

解决：略。

学习到的：

- map 的初始化，遍历
- strings 包的使用，包括转大小写，判断包含
- Go 中的字符串，rune 类型

经过 mentor review 后，被建议说可以用 switch...case 替代 map，性能有很大的提升，尝试后发现，果然，而且有一倍的提升，big suprise!

而且，多个相同 case 的写法是 `case 'A', 'E'...` 这样，而不是 `case 'A': case 'E': ...`

```go
switch c {
	case 'A', 'E', 'I', 'O', 'U':
		s = 1
	case 'B', 'D':
		s = 2
	...
}
```

- [Switch vs. Map: Which is the Better Way to Branch in Go?](https://hashrocket.com/blog/posts/switch-vs-map-which-is-the-better-way-to-branch-in-go)

### 6. Isogram

考查点：sequences, strings

问题：判断一个单词或句子里有无重复的字符

实现：略。

学习到的：

- 判断一个 rune 是否为字母：`unicode.IsLetter(r)`

通过 mentor 的 review，学习到了更简洁和巧妙的写法，并学习到了 map 判断元素是否存在的巧妙用法。

初版：

```go
func IsIsogram(s string) bool {
	chars := ""
	for _, c := range strings.ToLower(s) {
		if !unicode.IsLetter(c) {
			continue
		}
		if strings.ContainsRune(chars, c) {
			return false
		}
		chars += string(c)
	}
	return true
}
```

需要遍历两次，经过 mentor 指点后，改用 map 存储结果，使只需要遍历字符串一遍。第二版：

```go
func IsIsogram(s string) bool {
	chars := map[rune]int{}
	for _, c := range strings.ToLower(s) {
		if !unicode.IsLetter(c) {
			continue
		}
		_, ok := chars[c]
		if ok {
			return false
		}
		chars[c] = 1
	}
	return true
}
```

使用了 `map[rune]int` 存储结果，使用 `_, ok := chars[c]` 来判断元素是否已存在。mentor 指点说可以用 `map[rune]bool` 来使写法更简洁。原来这里有妙用，如果只使用一个变量来判断，如 `v := chars[c]`，且此元素不存在，会返回值的类型的默认值，比如 string 的默认值是 ""，bool 类型的默认值是 false。

所以，第三版：

```go
func IsIsogram(s string) bool {
	chars := map[rune]bool{}
	for _, c := range strings.ToLower(s) {
		if !unicode.IsLetter(c) {
			continue
		}
		exist := chars[c]
		if exist {
			return false
		}
		chars[c] = true
	}
	return true
}
```

妙！

### 7. Difference Of Squares

求 1~n 的和的平方，与平方的和的差。太简单，略过。

mentor review，告之有更简单的方法，可以不用循环，好吧，直接套数据公式。

- 和的平方：`((1+n)*n/2) * ((1+n)*n/2)`
- 平方之和：`n*(n+1)*(2n+1)/6`
