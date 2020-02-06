package romannumerals

import (
	"errors"
	"strings"
)

var letters = map[int]string{
	1:    "I",
	5:    "V",
	10:   "X",
	50:   "L",
	100:  "C",
	500:  "D",
	1000: "M",
}

func toRoman(num int, one string, five string, ten string) string {
	switch num {
	case 1:
		return one
	case 2:
		return one + one
	case 3:
		return one + one + one
	case 4:
		return one + five
	case 5:
		return five
	case 6:
		return five + one
	case 7:
		return five + one + one
	case 8:
		return five + one + one + one
	case 9:
		return one + ten
	default:
		return ""
	}
}

// num: 0~9
// decimal: 1, 10, 100, 1000
func toRomanSingleNumeral(num int, decimal int) (string, error) {
	if num < 0 || num > 9 {
		return "", errors.New("num can not less than 0 or greater than 9")
	}
	if decimal != 1 && decimal != 10 && decimal != 100 && decimal != 1000 {
		return "", errors.New("decimal only can be 1/10/100/1000")
	}
	switch decimal {
	case 1:
		return toRoman(num, letters[1], letters[5], letters[10]), nil
	case 10:
		return toRoman(num, letters[10], letters[50], letters[100]), nil
	case 100:
		return toRoman(num, letters[100], letters[500], letters[1000]), nil
	case 1000:
		m := letters[1000]
		switch num {
		case 1:
			return m, nil
		case 2:
			return m + m, nil
		case 3:
			return m + m + m, nil
		default:
			return "", errors.New("beyond the scope")
		}
	default:
		return "", errors.New("beyond the scope")
	}
}

// ToRomanNumeral converts normal numeral to RomanNumeral
func ToRomanNumeral(num int) (string, error) {
	if num <= 0 || num > 3000 {
		return "", errors.New("byond the scope")
	}

	roman := []string{}
	decimal := 1
	for num > 0 {
		digit := num % 10
		s, err := toRomanSingleNumeral(digit, decimal)
		if err != nil {
			return "", err
		}
		roman = append(roman, s)
		decimal *= 10
		num = num / 10
	}
	revRoman := make([]string, len(roman))
	for i := 0; i < len(roman); i++ {
		revRoman[len(roman)-i-1] = roman[i]
	}
	return strings.Join(revRoman, ""), nil
}
