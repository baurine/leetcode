package reverse

import (
	"fmt"
	"strings"
)

// Reverse reverses a string
func Reverse(src string) string {
	// 获得 str 中 rune 个数
	count := strings.Count(src, "") - 1
	runes := make([]rune, count)
	i := 0
	// fmt.Println(src)
	for _idx, c := range src {
		// 此处的 _idx 并非是 rune index，而是字节的 index
		// 所以还需要一个额外的 i 来记录 rune index
		fmt.Printf("%c -- %d\n", c, _idx)
		runes[count-i-1] = c
		i++
	}
	return string(runes)
}
