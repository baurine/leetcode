package accumulate

type maper func(string) string

// Accumulate converts source string slice by converter
func Accumulate(src []string, converter maper) []string {
	targets := make([]string, len(src))
	for i, s := range src {
		targets[i] = converter(s)
	}
	return targets
}

// This works as well
// Accumulate converts source string slice by converter
// func Accumulate(src []string, converter maper) (changed []string) {
// 	for _, s := range src {
// 		changed = append(changed, converter(s))
// 	}
// 	return
// }
