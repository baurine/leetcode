package scrabble

import "strings"

// Score calculates the scores of a string
func Score(s string) int {
	scoreTable := map[int]string{
		1:  "AEIOULNRST",
		2:  "DG",
		3:  "BCMP",
		4:  "FHVWY",
		5:  "K",
		8:  "JX",
		10: "QZ",
	}
	scores := 0
	for _, c := range strings.ToUpper(s) {
		for score, chars := range scoreTable {
			if strings.ContainsRune(chars, c) {
				scores += score
				continue
			}
		}
	}
	return scores
}
