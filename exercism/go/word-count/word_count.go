package wordcount

import (
	"strings"
	"unicode"
)

// Frequency represents how many times a word in a sentence
type Frequency map[string]int

// WordCount calculates the frequency of each word in the sentence
func WordCount(sentence string) Frequency {
	fre := Frequency{}
	words := strings.FieldsFunc(strings.ToLower(sentence), func(c rune) bool {
		return !unicode.IsLetter(c) && !unicode.IsNumber(c) && c != '\''
	})
	for _, w := range words {
		w = strings.Trim(w, "'")
		v, exist := fre[w]
		if exist {
			fre[w] = v + 1
		} else {
			fre[w] = 1
		}
	}
	return fre
}
