package scrabble

import "strings"

var scoreTable1 = map[int]string{
	1:  "AEIOULNRST",
	2:  "DG",
	3:  "BCMP",
	4:  "FHVWY",
	5:  "K",
	8:  "JX",
	10: "QZ",
}

// Score1 calculates the scores of a string
func Score1(s string) int {
	scores := 0
	for _, c := range strings.ToUpper(s) {
		for score, chars := range scoreTable1 {
			if strings.ContainsRune(chars, c) {
				scores += score
				continue
			}
		}
	}
	return scores
}

// === RUN   TestScore
// --- PASS: TestScore (0.00s)
// goos: darwin
// goarch: amd64
// pkg: scrabble
// BenchmarkScore-8          115317             10121 ns/op              80 B/op          9 allocs/op
// PASS
// ok      scrabble        1.597s

var scoreTable2 = map[rune]int{
	'A': 1,
	'E': 1,
	'I': 1,
	'O': 1,
	'U': 1,
	'L': 1,
	'N': 1,
	'R': 1,
	'S': 1,
	'T': 1,
	'D': 2,
	'G': 2,
	'B': 3,
	'C': 3,
	'M': 3,
	'P': 3,
	'F': 4,
	'H': 4,
	'V': 4,
	'W': 4,
	'Y': 4,
	'K': 5,
	'J': 8,
	'X': 8,
	'Q': 10,
	'Z': 10,
}

// Score2 calculates the scores of a string
func Score2(s string) int {
	scores := 0
	for _, c := range strings.ToUpper(s) {
		scores += scoreTable2[c]
	}
	return scores
}

// === RUN   TestScore
// --- PASS: TestScore (0.00s)
// goos: darwin
// goarch: amd64
// pkg: scrabble
// BenchmarkScore-8         1227236               987 ns/op              80 B/op          9 allocs/op
// PASS
// ok      scrabble        6.893s

// Score calculates the scores of a string
func Score(src string) int {
	scores := 0
	for _, c := range strings.ToUpper(src) {
		var s int
		switch c {
		case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T':
			s = 1
		case 'D', 'G':
			s = 2
		case 'B', 'C', 'M', 'P':
			s = 3
		case 'F', 'H', 'V', 'W', 'Y':
			s = 4
		case 'K':
			s = 5
		case 'J', 'X':
			s = 8
		case 'Q', 'Z':
			s = 10
		}
		scores += s
	}
	return scores
}

// === RUN   TestScore
// --- PASS: TestScore (0.00s)
// goos: darwin
// goarch: amd64
// pkg: scrabble
// BenchmarkScore-8         2434929               487 ns/op              80 B/op          9 allocs/op
// PASS
// ok      scrabble        2.038s
