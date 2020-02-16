package collatzconjecture

import "errors"

// 递归
func seqLenRecursion(n int) int {
	if n == 1 {
		return 0
	}
	if n%2 == 0 {
		n = n / 2
	} else {
		n = 3*n + 1
	}
	return seqLenRecursion(n) + 1
}

// CollatzConjecture compute how many steps the input will become 1
func CollatzConjecture(input int) (int, error) {
	if input <= 0 {
		return 0, errors.New("input must be greater than 0")
	}
	return seqLenRecursion(input), nil
}
