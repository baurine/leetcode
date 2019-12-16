package euler

import (
	"testing"
)

func TestEuler14(t *testing.T) {
	startNum, len := Euler14()
	if startNum == 837799 && len == 525 {
		t.Log("ok")
	} else {
		t.Fatal("failed")
	}
}
