package space

// Planet is the name of the planet
type Planet string

const earthYearSecond float64 = 31557600

var factors = map[Planet]float64{
	"Mercury": 0.2408467,
	"Venus":   0.61519726,
	"Earth":   1.0,
	"Mars":    1.8808158,
	"Jupiter": 11.862615,
	"Saturn":  29.447498,
	"Uranus":  84.016846,
	"Neptune": 164.79132,
}

// Age calculates the age in different planet
func Age(seconds float64, planet Planet) float64 {
	// 先计算得到地球上的时间
	age := seconds / earthYearSecond
	factor, ok := factors[planet]
	if ok {
		age = age / factor
	}
	return age
}
