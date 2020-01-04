package euler

import (
	"testing"
)

// Equal tells whether a and b contain the same elements.
// A nil argument is equivalent to an empty slice.
// TODO: extract
func equal(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}

func TestDivisors(t *testing.T) {
	cases := []struct {
		in       int
		expected []int
	}{
		{1, []int{1}},
		{2, []int{1}},
		{3, []int{1}},
		{4, []int{1, 2}},
		{5, []int{1}},
		{6, []int{1, 2, 3}},
		{220, []int{1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110}},
		{284, []int{1, 2, 4, 71, 142}},
	}

	for _, tt := range cases {
		actual := divisors(tt.in)
		if !equal(actual, tt.expected) {
			t.Errorf("%d: %d, expected: %d", tt.in, actual, tt.expected)
		}
	}
}

func TestDivisorsSum(t *testing.T) {
	cases := []struct {
		in       int
		expected int
	}{
		{1, 1},
		{2, 1},
		{3, 1},
		{4, 3},
		{5, 1},
		{6, 6},
		{220, 284},
		{284, 220},
	}

	for _, tt := range cases {
		actual := divisorsSum(tt.in)
		if actual != tt.expected {
			t.Errorf("%d: %d, expected: %d", tt.in, actual, tt.expected)
		}
	}
}

func TestEuler21(t *testing.T) {
	if 31626 != Euler21() {
		t.Error("failed")
	}
}
