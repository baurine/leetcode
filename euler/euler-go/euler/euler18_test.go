package euler

import (
	"testing"
)

func TestRow(t *testing.T) {
	cases := []struct {
		in       int
		expected int
	}{
		{0, 1},
		{1, 2},
		{2, 2},
		{17, 6},
	}

	for _, tt := range cases {
		actual := row(tt.in)
		if actual != tt.expected {
			t.Errorf("%d: %d, expected: %d", tt.in, actual, tt.expected)
		}
	}
}

func TestEuler18(t *testing.T) {
	if 1074 != Euler18() {
		t.Error("failed")
	}
}
