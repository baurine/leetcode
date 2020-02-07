// Package bob handles bob's reponse
package bob

import "strings"

const (
	emptyReply          = "Fine. Be that way!"
	questonReply        = "Sure."
	yellReply           = "Whoa, chill out!"
	questionYellReply   = "Calm down, I know what I'm doing!"
	othersReply         = "Whatever."
	allUppercaseLetters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
)

// Hey returns bob's response to what talked to him
func Hey(remark string) string {
	r := strings.TrimSpace(remark)
	empty := len(r) == 0
	question := strings.HasSuffix(r, "?")
	yell := strings.ToUpper(r) == r && strings.ContainsAny(r, allUppercaseLetters)

	switch {
	case empty:
		return emptyReply
	case question && yell:
		return questionYellReply
	case question:
		return questonReply
	case yell:
		return yellReply
	default:
		return othersReply
	}
}
