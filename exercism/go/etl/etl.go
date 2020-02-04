package etl

import "strings"

// Transform converts the data format
func Transform(src map[int][]string) map[string]int {
	target := map[string]int{}
	for score, s := range src {
		for _, c := range s {
			target[strings.ToLower(c)] = score
		}
	}
	return target
}
