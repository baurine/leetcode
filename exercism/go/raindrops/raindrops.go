package raindrops

import "strconv"

// Convert converts the number to raindrop sound
func Convert(n int) string {
	var sound string
	if n%3 == 0 {
		sound += "Pling"
	}
	if n%5 == 0 {
		sound += "Plang"
	}
	if n%7 == 0 {
		sound += "Plong"
	}
	if sound == "" {
		sound = strconv.Itoa(n)
	}
	return sound
}
