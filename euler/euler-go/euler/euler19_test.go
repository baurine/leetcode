package euler

import (
	"testing"
)

func TestLearYear(t *testing.T) {
	cases := []struct {
		in       int
		expected bool
	}{
		{1900, false},
		{2000, true},
		{1904, true},
		{2004, true},
		{2100, false},
		{2019, false},
		{2020, true},
	}

	for _, tt := range cases {
		actual := leapYear(tt.in)
		if actual != tt.expected {
			t.Errorf("%d: %t, expected: %t", tt.in, actual, tt.expected)
		}
	}
}

func TestEuler19(t *testing.T) {
	if 171 != Euler19() {
		t.Error("failed")
	}
}
