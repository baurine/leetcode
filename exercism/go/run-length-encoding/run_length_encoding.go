package encode

import (
	"fmt"
	"strconv"
	"unicode"
)

// RunLengthEncode is a simple form of data compression
// where runs are replaced by just one data value and count.
func RunLengthEncode(plain string) string {
	if len(plain) == 0 {
		return ""
	}
	var output string
	var pre rune
	var count int
	for i, v := range plain {
		if i == 0 {
			pre = v
			count = 1
			continue
		}
		if v == pre {
			count++
		} else {
			if count == 1 {
				output += string(pre)
			} else {
				output += fmt.Sprintf("%d%c", count, pre)
			}
			pre = v
			count = 1
		}
	}
	if count == 1 {
		output += string(pre)
	} else {
		output += fmt.Sprintf("%d%c", count, pre)
	}
	return output
}

// RunLengthDecode decodes the compressed data by RunLengthEncode
func RunLengthDecode(encoded string) string {
	if len(encoded) == 0 {
		return ""
	}
	var plain string
	lastCharIdx := -1 // make -1 as initial value is the keypoint
	for i, v := range encoded {
		if !unicode.IsNumber(v) {
			if lastCharIdx == i-1 {
				plain += string(v)
			} else {
				num := encoded[lastCharIdx+1 : i]
				count, _ := strconv.Atoi(num)
				for j := 0; j < count; j++ {
					plain += string(v)
				}
			}
			lastCharIdx = i
		}
	}
	return plain
}
