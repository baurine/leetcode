package euler

import (
	"testing"
)

func TestEuler15(t *testing.T) {
	if Euler15() == 137846528820 {
		t.Log("ok")
	} else {
		t.Error("failed")
	}
}
