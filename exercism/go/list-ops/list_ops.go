package listops

type binFunc func(x, y int) int

// IntList represets the int slice
type IntList []int

// Foldl accumulates the IntList from left to right
// accumulate value is the binFunc first argument
func (il IntList) Foldl(fn binFunc, initial int) int {
	accu := initial
	for _, v := range il {
		accu = fn(accu, v)
	}
	return accu
}

// Foldr accumulates the IntList from right to left
// accumulate value is the binFunc second argument
func (il IntList) Foldr(fn binFunc, initial int) int {
	accu := initial
	for i := il.Length() - 1; i >= 0; i-- {
		accu = fn((il)[i], accu)
	}
	return accu
}

///////////////////

type predFunc func(int) bool

// Filter selects the values from IntList whose predFunc is true
func (il IntList) Filter(fn predFunc) IntList {
	ret := IntList{}
	for _, v := range il {
		if fn(v) {
			ret = append(ret, v)
		}
	}
	return ret
}

///////////////////

// Length returns IntList length
func (il IntList) Length() int {
	return len(il)
}

///////////////////

type unaryFunc func(int) int

// Map convert IntList each value to another value by unaryFunc
func (il IntList) Map(fn unaryFunc) IntList {
	ret := IntList{}
	for _, v := range il {
		ret = append(ret, fn(v))
	}
	return ret
}

///////////////////

// Reverse reverses the IntList values
func (il IntList) Reverse() IntList {
	ret := make(IntList, il.Length())
	for i, v := range il {
		ret[il.Length()-i-1] = v
	}
	return ret
}

// Append appends another IntList in the tail
func (il IntList) Append(ail IntList) IntList {
	ret := IntList(il)
	for _, v := range ail {
		ret = append(ret, v)
	}
	return ret
}

// Concat appends IntList slice int the tail
func (il IntList) Concat(ails []IntList) IntList {
	ret := IntList(il)
	for _, il := range ails {
		for _, v := range il {
			ret = append(ret, v)
		}
	}
	return ret
}
