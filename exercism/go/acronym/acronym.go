// Package acronym handles string's acronym
package acronym

import "strings"

// Abbreviate returns acronym of a string
func Abbreviate(s string) string {
	f := func(c rune) bool {
		return c == ' ' || c == '-' || c == '_'
	}
	slice := strings.FieldsFunc(s, f)
	capitals := make([]string, len(slice))
	for i, str := range slice {
		capitals[i] = string(strings.ToUpper(str)[0])
	}
	return strings.Join(capitals, "")
}
