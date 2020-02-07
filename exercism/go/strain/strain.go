package strain

// Ints represents int slice
type Ints []int

// Lists represents int slice slice
type Lists [][]int

// Strings represents string slice
type Strings []string

// Keep select the items where the predicate is true
func (src Ints) Keep(predicate func(int) bool) (output Ints) {
	for _, item := range src {
		if predicate(item) {
			output = append(output, item)
		}
	}
	return
}

// Discard select the items where the predicate is false
func (src Ints) Discard(predicate func(int) bool) (output Ints) {
	for _, item := range src {
		if !predicate(item) {
			output = append(output, item)
		}
	}
	return
}

// Keep select the items where the predicate is true
func (src Lists) Keep(predicate func([]int) bool) (output Lists) {
	for _, item := range src {
		if predicate(item) {
			output = append(output, item)
		}
	}
	return
}

// Keep select the items where the predicate is true
func (src Strings) Keep(predicate func(string) bool) (output Strings) {
	for _, item := range src {
		if predicate(item) {
			output = append(output, item)
		}
	}
	return
}
