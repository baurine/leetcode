package euler

import (
	"testing"
)

func TestEuler20(t *testing.T) {
	if 648 != Euler20() {
		t.Error("failed")
	}
}
