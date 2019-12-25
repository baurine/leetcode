package euler

import (
	"fmt"
	"strings"
)

func numberToWords(num int) string {
	if num >= 1 && num <= 9 {
		words := []string{"", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
		return words[num]
	} else if num >= 10 && num <= 19 {
		words := []string{"ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"}
		return words[num%10]
	} else if num >= 20 && num <= 99 {
		if num%10 == 0 {
			words := []string{"", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"}
			return words[num/10]
		} else {
			return fmt.Sprintf("%s %s", numberToWords(num-num%10), numberToWords(num%10))
		}
	} else if num >= 100 && num <= 999 {
		if num%100 == 0 {
			return fmt.Sprintf("%s hundred", numberToWords(num/100))
		} else {
			return fmt.Sprintf("%s hundred and %s", numberToWords(num/100), numberToWords(num%100))
		}
	} else if num == 1000 {
		return "one thousand"
	} else {
		return ""
	}
}

func numberWordsLen(num int) int {
	words := numberToWords(num)
	wordsArr := strings.Split(words, " ")
	l := 0
	for _, word := range wordsArr {
		l += len(word)
	}
	return l
}

func Euler17() int {
	totalLen := 0
	for i := 1; i <= 1000; i++ {
		totalLen += numberWordsLen(i)
	}
	return totalLen
}
