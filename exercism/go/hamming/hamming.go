package hamming

import "errors"

// Distance of 2 same length DNA sequences
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("lens is different")
	}
	diffCount := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			diffCount++
		}
	}
	return diffCount, nil
}
