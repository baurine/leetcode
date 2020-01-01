package euler

import "time"

func leapYear(y int) bool {
	if y%400 == 0 {
		return true
	} else if y%100 == 0 {
		return false
	} else if y%4 == 0 {
		return true
	} else {
		return false
	}
}

func Euler19() int {
	leapYearMonths := [...]int{0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	nonLeapYearMonths := [...]int{0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	count := 0
	accDays := 0 // 从 1900.1.1 号开始的累积天数，这一天是周一
	for y := 1900; y <= 2000; y++ {
		for m := 1; m <= 12; m++ {
			weekDay := (accDays + 1) % 7
			if weekDay == 0 && y > 1900 {
				count += 1
			}
			if leapYear(y) {
				accDays += leapYearMonths[m]
			} else {
				accDays += nonLeapYearMonths[m]
			}
		}
	}
	return count
}

func Euler19_2() int {
	count := 0
	for y := 1901; y <= 2000; y++ {
		for m := time.January; m <= time.December; m++ {
			d := time.Date(y, m, 1, 0, 0, 0, 0, time.UTC)
			if d.Weekday() == time.Sunday && d.Day() == 1 {
				count += 1
			}
		}
	}
	return count
}
