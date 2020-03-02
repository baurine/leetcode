package proverb

import "fmt"

// Proverb generates the relevant proverb by given a list of inputs
func Proverb(rhyme []string) []string {
	output := []string{}
	if len(rhyme) == 0 {
		return output
	}
	if len(rhyme) >= 2 {
		for i := 0; i < len(rhyme)-1; i++ {
			output = append(output, fmt.Sprintf("For want of a %s the %s was lost.", rhyme[i], rhyme[i+1]))
		}
	}
	output = append(output, fmt.Sprintf("And all for the want of a %s.", rhyme[0]))
	return output
}
