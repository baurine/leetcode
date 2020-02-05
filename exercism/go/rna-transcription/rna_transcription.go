package strand

import "bytes"

// ToRNA transcript DNA strand
func ToRNA(dna string) string {
	var buffer bytes.Buffer
	var r rune
	for _, c := range dna {
		switch c {
		case 'A':
			r = 'U'
		case 'T':
			r = 'A'
		case 'C':
			r = 'G'
		case 'G':
			r = 'C'
		default:
			r = c
		}
		buffer.WriteRune(r)
	}
	return buffer.String()
}
