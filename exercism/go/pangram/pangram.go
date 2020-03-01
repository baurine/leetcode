package pangram

import (
	"strings"
	"unicode"
)

// IsPangram determines if a sentence using every letter of the alphabet at least once
func IsPangram(input string) bool {
	seen := make([]bool, 26)
	for _, c := range strings.ToLower(input) {
		if unicode.IsLetter(c) {
			seen[c-'a'] = true
		}
	}
	for _, s := range seen {
		if !s {
			return false
		}
	}
	return true
}
