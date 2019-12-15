package euler

import (
	"testing"
)

func TestEuler1(t *testing.T) {
	sum := Euler1()
	if sum == 233168 {
		t.Log("ok")
	} else {
		t.Fatal("failed")
	}
}
