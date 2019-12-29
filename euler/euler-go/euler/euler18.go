package euler

func row(n int) int {
	s := 0
	for row := 1; row >= 1; row++ {
		s += row
		if s > n {
			return row
		}
	}
	return 0
}

func maxPathWeight(w []int, n int) int {
	if n >= len(w) {
		return 0
	}
	curRow := row(n)
	lastRow := row(len(w) - 1)
	if curRow == lastRow {
		return w[n] // 退出条件，当前已是最后一行
	}
	left := maxPathWeight(w, n+curRow)
	right := maxPathWeight(w, n+curRow+1)
	max := left
	if right > left {
		max = right
	}
	return w[n] + max
}

func Euler18() int {
	w := []int{
		75,     // row 1
		95, 64, //row2
		17, 47, 82, //row3
		18, 35, 87, 10, // row4
		20, 04, 82, 47, 65, // row5
		19, 01, 23, 75, 03, 34, // row6
		88, 02, 77, 73, 07, 63, 67, // row7
		99, 65, 04, 28, 06, 16, 70, 92, // row8
		41, 41, 26, 56, 83, 40, 80, 70, 33, // row9
		41, 48, 72, 33, 47, 32, 37, 16, 94, 29, // row10
		53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, // row11
		70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, // row12
		91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, // row13
		63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, // row14
		04, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 04, 23, // row15
	}
	return maxPathWeight(w, 0)
}
