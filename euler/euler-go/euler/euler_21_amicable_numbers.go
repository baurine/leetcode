package euler

import (
	"math"
	"sort"
)

func divisors(n int) []int {
	v := []int{}
	for i := 1; i <= int(math.Floor(math.Sqrt(float64(n)))); i++ {
		if n%i == 0 {
			v = append(v, i)

			divisor := n / i
			if divisor > i && divisor < n {
				v = append(v, divisor)
			}
		}
	}
	sort.Ints(v)
	return v
}

func divisorsSum(n int) int {
	sum := 0
	for _, x := range divisors(n) {
		sum += x
	}
	return sum
}

func Euler21() int {
	totalSum := 0
	for i := 1; i < 10000; i++ {
		sum := divisorsSum(i)
		if sum > i && divisorsSum(sum) == i {
			totalSum += (i + sum)
		}
	}
	return totalSum
}
