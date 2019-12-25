package euler

import (
	"testing"
)

// Go 可以测试私有方法，只要它们在同一个包下
func TestNumberToWords(t *testing.T) {
	// 表组测试
	// https://books.studygolang.com/The-Golang-Standard-Library-by-Example/chapter09/09.1.html#table-driven-test
	cases := []struct {
		in       int
		expected string
	}{
		{1, "one"},
		{2, "two"},
		{12, "twelve"},
		{50, "fifty"},
		{54, "fifty four"},
		{600, "six hundred"},
		{654, "six hundred and fifty four"},
	}

	for _, tt := range cases {
		actual := numberToWords(tt.in)
		if actual != tt.expected {
			t.Errorf("%d: %s, expected: %s", tt.in, actual, tt.expected)
		}
	}
}
func TestNumberWordsLen(t *testing.T) {
	cases := []struct {
		in       int
		expected int
	}{
		{1, 3},
		{2, 3},
		{12, 6},
		{50, 5},
		{54, 9},
		{600, 10},
		{654, 22},
	}
	for _, tt := range cases {
		actual := numberWordsLen(tt.in)
		if actual != tt.expected {
			t.Errorf("%d: %d, expected: %d", tt.in, actual, tt.expected)
		}
	}
}

func TestEuler17(t *testing.T) {
	if 21124 != Euler17() {
		t.Error("failed")
	}
}
