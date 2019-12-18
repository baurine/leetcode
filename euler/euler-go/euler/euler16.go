package euler

func powSum(pow int) int {
	if pow == 1 {
		return 2
	}
	v := []int{2}
	carry := 0
	for i := 2; i <= pow; i++ {
		carry = 0
		for idx, x := range v {
			x = x*2 + carry
			v[idx] = x % 10
			carry = x / 10
		}
		if carry > 0 {
			v = append(v, carry)
		}
	}
	sum := 0
	for _, x := range v {
		sum += x
	}
	return sum
}

func Euler16() int {
	return powSum(1000)
}
