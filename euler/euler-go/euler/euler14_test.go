package euler

import (
	"testing"
)

func TestEuler14(t *testing.T) {
	solutionTypes := [...]int{1, 2, 3, 4}
	for _, x := range solutionTypes {
		startNum, len := Euler14(x)
		if startNum == 837799 && len == 525 {
			t.Log("ok")
		} else {
			t.Errorf("failed when %d, fact: startNum: %d, len: %d", x, startNum, len)
		}
	}
}

func BenchmarkEuler14_1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Euler14(1)
	}
}

func BenchmarkEuler14_2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Euler14(2)
	}
}

func BenchmarkEuler14_3(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Euler14(3)
	}
}

func BenchmarkEuler14_4(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Euler14(4)
	}
}

// benchmark 结果，看来此例中缓存的作用并不是很大，还有可能是负作用，而且递归的效率是远低于迭代的。
// 主要是缓存命中率较低，反而多出来额外的对缓存的操作
// > go test -bench=.
// goos: darwin
// goarch: amd64
// pkg: euler-go/euler
// BenchmarkEuler14_1-8           4         252275802 ns/op
// BenchmarkEuler14_2-8           2         553177518 ns/op
// BenchmarkEuler14_3-8           2         578678881 ns/op
// BenchmarkEuler14_4-8           2         530781044 ns/op
