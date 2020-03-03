package encode

import (
	"strconv"
	"strings"
	"unicode"
)

// RunLengthEncode is a simple form of data compression
// where runs are replaced by just one data value and count.
func RunLengthEncode(plain string) string {
	if len(plain) == 0 {
		return ""
	}
	output := &strings.Builder{}
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
			if count > 1 {
				output.WriteString(strconv.Itoa(count))
			}
			output.WriteRune(pre)
			pre = v
			count = 1
		}
	}
	if count > 1 {
		output.WriteString(strconv.Itoa(count))
	}
	output.WriteRune(pre)
	return output.String()
}

// RunLengthDecode decodes the compressed data by RunLengthEncode
func RunLengthDecode(encoded string) string {
	if len(encoded) == 0 {
		return ""
	}
	plain := &strings.Builder{}
	lastCharIdx := -1 // make -1 as initial value is the keypoint
	for i, v := range encoded {
		if !unicode.IsNumber(v) {
			if lastCharIdx == i-1 {
				plain.WriteRune(v)
			} else {
				num := encoded[lastCharIdx+1 : i]
				count, _ := strconv.Atoi(num)
				plain.WriteString(strings.Repeat(string(v), count))
			}
			lastCharIdx = i
		}
	}
	return plain.String()
}
