package scale

import "strings"

var (
	sharpNotes = []string{
		"A",
		"A#",
		"B",
		"C",
		"C#",
		"D",
		"D#",
		"E",
		"F",
		"F#",
		"G",
		"G#",
	}
	flatNotes = []string{
		"A",
		"Bb",
		"B",
		"C",
		"Db",
		"D",
		"Eb",
		"E",
		"F",
		"Gb",
		"G",
		"Ab",
	}
	// can be replaced by switch...case
	choices = map[string][]string{
		"C":  sharpNotes,
		"G":  sharpNotes,
		"D":  sharpNotes,
		"A":  sharpNotes,
		"E":  sharpNotes,
		"B":  sharpNotes,
		"F#": sharpNotes,
		"e":  sharpNotes,
		"b":  sharpNotes,
		"f#": sharpNotes,
		"c#": sharpNotes,
		"g#": sharpNotes,
		"d#": sharpNotes,
		"a":  sharpNotes,

		"F":  flatNotes,
		"Bb": flatNotes,
		"Eb": flatNotes,
		"Ab": flatNotes,
		"Db": flatNotes,
		"Gb": flatNotes,
		"d":  flatNotes,
		"g":  flatNotes,
		"c":  flatNotes,
		"f":  flatNotes,
		"bb": flatNotes,
		"eb": flatNotes,
	}
)

// Scale generates the musical scale by given starting note and a set of intervals
func Scale(startNote string, intervals string) []string {
	scales := []string{}
	notes := choices[startNote]

	// convert, upper the first letter, for example "eb" -> "Eb"
	// note := strings.ToUpper(string(startNote[0]))
	// if len(startNote) > 1 {
	// 	note += string(startNote[1:])
	// }
	// strings.Title() converts each word's first letter to uppercase
	note := strings.Title(startNote)

	// find the index which note in the notes
	var idx int
	for i, v := range notes {
		if v == note {
			idx = i
			break
		}
	}
	scales = append(scales, notes[idx])

	steps := intervals
	if len(steps) == 0 {
		steps = "mmmmmmmmmmm"
	} else {
		steps = steps[0 : len(steps)-1]
	}
	for _, r := range steps {
		switch r {
		case 'm':
			idx++
		case 'M':
			idx += 2
		case 'A':
			idx += 3
		}
		scales = append(scales, notes[idx%len(notes)])
	}

	return scales
}
