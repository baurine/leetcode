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

### ETL

考查点：maps, loops, transforming

问题：将 int -> string 的映射关系变成 char -> int

解决：双重循环即可，详略。

学习到的：

- Go 中 map 的初始化，遍历

### RNA Transcrption

考查点：maps, transforming

问题：将 DNA 转换成 RAN 序列

解决：略。

学习到的：

- bytes.Buffer, buffer.WriteRune(), buffer.String()
- switch...case，不需要加 break

看了一下社区解决方案，看到一个极简解法：

```go
func ToRNA(dna string) string {
	return strings.NewReplacer(
		"G", "C",
		"C", "G",
		"T", "A",
		"A", "U",
	).Replace(dna)
}
```

还有这样的：

```go
var transcription = map[byte]byte{
	'G': 'C',
	'C': 'G',
	'T': 'A',
	'A': 'U',
}

func ToRNA(dna string) string {
	var rna []byte
	for i := range dna {
		rna = append(rna, transcription[dna[i]])
	}
	return string(rna)
}
```

### Roman Numerals

考查点：maps, transforming

问题：将阿拉伯数字转换成罗马数字，比如 27 -> XXVII (倒是终于知道了罗马数字是怎么计数的了)

- [On Roman Numerals](http://www.novaroma.org/via_romana/numbers.html)

```go
var letters = map[int]string{
	1:    "I",
	5:    "V",
	10:   "X",
	50:   "L",
	100:  "C",
	500:  "D",
	1000: "M",
}
```

`IV` 是 4，`VI` 是 6，`IX` 是 10，`XI` 是 11，依此类推。

解决：我的解法，常规解法，按规则逐位解析再拼接。洋洋洒洒写了 100 行，结果一看社区方案，第一个就看到一个很机智的解法，才 20 行，惊呆我了，人才啊。

```go
var arabics = []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
var romans = []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}

// ToRomanNumeral converts arabic numbers to roman numerals
func ToRomanNumeral(arabic int) (roman string, err error) {
	if !(arabic > 0 && arabic <= 3000) {
		return roman, errors.New("number is out of range")
	}
	for i := 0; i < len(arabics); i++ {
		for arabic >= arabics[i] {
			arabic -= arabics[i]
			roman += romans[i]
		}
	}
	return roman, nil
}
```

不过总的来说，Go 还是一门比较朴素的语言，写起来就像写 C 一样，没那么多花样，挺好的。

### Strain

考查点：arrays, filtering, loops

问题：为 `[]int` `[]string` `[][]int` 实现类似 select/reject 的 Keep/Discard 方法。

实现：略。

从这看出了 Go 没有泛型的不便，相同的方法和代码要写三遍。

### Nucleotide Count

考查点：maps, parsing, strings

问题：统计一段 DNA 序列中 A/C/G/T 的个数。

实现：略。

### Bob

考查点：conditional, strings

问题：你朝 bob 说不同的话，它会进行不同的回复。

解决：略。在 Rust 中做过了相同的 track，直接把逻辑复制过来了。

可以用 switch...case 代替多个 if...else if...else if...else

清除左右的空格可以用 strings.TrimSpace()

判断一个全大写字符串里至少有一个字符我用了简单粗暴的 `strings.ContainsAny(s, "ABCDEFGHIJKLMNOPQRSTUVWXYZ")`，看了社区方案，很多用了 `unicode.IsLetter(c)`。

### Acronym

考查点：regex, strings (实际我并没有使用 regex)

问题：返回一段文本串的首字母缩写。

解决：略。

使用到了 strings.FieldsFunc() 方法进行 split。

最近这些题全是各种处理字符串，主要是 strings 包的使用。

### Reverse String

考查点：sequences, strings

问题：反转字符串。(包括中文)

学习到的：

- 获取字符串中 rune 的个数：`count := strings.Count(src, "") - 1`
- 在对 string 进行 rune 遍历时，即 `for _idx, c := range src {...}`，此处的 `_idx` 并不是 rune index，而是 byte index，所以为了记录 rune 的索引，还需要额外使用一个变量，比如 i

在社区中看到的另一种方案：

```go
//Reverse takes a string and reverses it
func Reverse(input string) string {
	if input != "" {
		bString := []rune(input)
		var rString = make([]rune, 0)
		for i := len(bString) - 1; i >= 0; i-- {
			rString = append(rString, bString[i])
		}
		return string(rString)
	} else {
		return input
	}
}
```

先通过 `bString := []rune(input)` 将 string 转换成了 rune slice。但我觉得这样占用额外的内存空间。
