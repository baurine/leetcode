package diffsquares

// SquareOfSum calculates the square of the sum of the first N natural numbers.
func SquareOfSum(n int) int {
	sum := (1 + n) * n / 2
	return sum * sum
}

// SumOfSquares calculates the sum of the squares of first N natural numbers.
func SumOfSquares(n int) int {
	// https://blog.csdn.net/tina_tian1/article/details/55209890
	return n * (n + 1) * (2*n + 1) / 6
}

// Difference calculates the difference between the square of the sum and the sum of the squares of the first N natural numbers.
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
