package darts

// Score calculate the darts game score by its position
func Score(x, y float64) int {
	squareSum := x*x + y*y
	switch {
	case squareSum > 10.0*10.0:
		return 0
	case squareSum > 5.0*5.0:
		return 1
	case squareSum > 1.0*1.0:
		return 5
	default:
		return 10
	}
}
