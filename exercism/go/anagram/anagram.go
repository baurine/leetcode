package anagram

import (
	"sort"
	"strings"
)

func sortString(input string) string {
	strSlices := strings.Split(input, "")
	sort.Strings(strSlices)
	return strings.Join(strSlices, "")
}

// Detect checks whether a word is a rearrangement of letters to form a new word
func Detect(input string, candiates []string) []string {
	var output []string

	lowerInput := strings.ToLower(input)
	sortedInput := sortString(lowerInput)

	for _, s := range candiates {
		if len(input) != len(s) {
			continue
		}
		if lowerInput == strings.ToLower(s) {
			continue
		}
		if sortedInput == sortString(strings.ToLower(s)) {
			output = append(output, s)
		}
	}

	return output
}
