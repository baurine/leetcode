package isogram

import (
	"strings"
	"unicode"
)

// IsIsogram checks whether a word or a pharse is isogram
func IsIsogram(s string) bool {
	seen := map[rune]bool{}
	for _, c := range strings.ToLower(s) {
		if !unicode.IsLetter(c) {
			continue
		}
		if seen[c] {
			return false
		}
		seen[c] = true
	}
	return true
}
