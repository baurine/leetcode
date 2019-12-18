package euler

// import (
// 	"fmt"
// )

// 迭代
func seqLen(n int) int {
	len := 1
	for n > 1 {
		if n%2 == 0 {
			n = n / 2
		} else {
			n = 3*n + 1
		}
		len++
	}
	return len
}

// 迭代 + 缓存 (只缓存住了部分)
func seqLenCached(n int, cache map[int]int) int {
	len := 1
	next := n
	for next > 1 {
		cached_len, ok := cache[next]
		if ok {
			len = len + cached_len - 1
			break
		}
		if next%2 == 0 {
			next = next / 2
		} else {
			next = 3*next + 1
		}
		len++
	}
	cache[n] = len
	return len
}

// 递归
func seqLenRecursion(n int) int {
	if n == 1 {
		return 1
	}
	if n%2 == 0 {
		n = n / 2
	} else {
		n = 3*n + 1
	}
	return seqLenRecursion(n) + 1
}

// 递归 + 缓存 (全缓存)
func seqLenRecursionCached(n int, cache map[int]int) int {
	if n == 1 {
		return 1
	}

	len, ok := cache[n]
	if ok {
		return len
	}

	next := n
	if n%2 == 0 {
		next = n / 2
	} else {
		next = 3*n + 1
	}
	len = seqLenRecursionCached(next, cache) + 1
	cache[n] = len
	return len
}

func Euler14(solutionType int) (int, int) {
	maxSeqLenStartNum := 0
	maxSeqLen := 0
	cache := make(map[int]int)
	curLen := 0
	for i := 1; i < 100_0000; i++ {
		if solutionType == 1 {
			curLen = seqLen(i)
		} else if solutionType == 2 {
			curLen = seqLenCached(i, cache)
		} else if solutionType == 3 {
			curLen = seqLenRecursion(i)
		} else {
			curLen = seqLenRecursionCached(i, cache)
		}
		if curLen > maxSeqLen {
			maxSeqLen = curLen
			maxSeqLenStartNum = i
		}
	}
	// fmt.Printf("\n%d -- cache len: %d\n", solutionType, len(cache))
	return maxSeqLenStartNum, maxSeqLen
}
