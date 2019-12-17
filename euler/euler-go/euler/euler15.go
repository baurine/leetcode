package euler

func pathSlow(v []int, m int, n int) int {
	if m == 0 || n == 0 {
		return 1
	}
	if v[m*100+n] > 0 {
		return v[m*100+n]
	}
	sum := 0
	for i := 0; i <= n; i++ {
		sum += pathSlow(v, m-1, i)
	}
	v[m*100+n] = sum
	return sum
}

func Euler15() int {
	v := make([]int, 100*100)
	return pathSlow(v, 20, 20)
}
