package euler

/*
问题：100! 的结果的每一位相加的知
解题思路：和 16 题类似，用切片存储阶乘结果的每一位
*/

func factorialNBits(n int) []int {
	bits := []int{1}
	var carry int
	for i := 1; i <= n; i++ {
		carry = 0
		for idx, bit := range bits {
			bit = bit*i + carry
			bits[idx] = bit % 10
			carry = bit / 10
		}
		for carry > 0 {
			bits = append(bits, carry%10)
			carry = carry / 10
		}
	}
	return bits
}

func Euler20() int {
	arr := factorialNBits(100)
	sum := 0
	for _, bit := range arr {
		sum += bit
	}
	return sum
}
