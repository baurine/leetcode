package isogram

import (
	"strings"
	"unicode"
)

// IsIsogram checks whether a word or a pharse is isogram
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
