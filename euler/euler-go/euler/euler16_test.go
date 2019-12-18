package euler

import (
	"testing"
)

func TestEuler16(t *testing.T) {
	if Euler16() == 1366 {
		t.Log("ok")
	} else {
		t.Error("failed")
	}
}
