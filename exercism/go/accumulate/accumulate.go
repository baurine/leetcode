package accumulate

// Accumulate converts source string slice by converter
func Accumulate(src []string, converter func(string) string) []string {
	targets := make([]string, len(src))
	for i, s := range src {
		targets[i] = converter(s)
	}
	return targets
}
