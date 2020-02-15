package isogram

import (
	"strings"
	"unicode"
)

// IsIsogram checks whether a word or a pharse is isogram
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
