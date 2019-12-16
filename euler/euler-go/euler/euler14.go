package euler

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

func Euler14() (int, int) {
	maxSeqLenStartNum := 0
	maxSeqLen := 0
	for i := 1; i < 100_0000; i++ {
		curLen := seqLen(i)
		if curLen > maxSeqLen {
			maxSeqLen = curLen
			maxSeqLenStartNum = i
		}
	}
	return maxSeqLenStartNum, maxSeqLen
}
