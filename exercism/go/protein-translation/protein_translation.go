package protein

import "errors"

// FromCodon err type
var (
	ErrStop        = errors.New("stop")
	ErrInvalidBase = errors.New("invalid")
)

// FromCodon translate codon to protein
func FromCodon(codon string) (protein string, err error) {
	switch codon {
	case "AUG":
		protein = "Methionine"
	case "UUU", "UUC":
		protein = "Phenylalanine"
	case "UUA", "UUG":
		protein = "Leucine"
	case "UCU", "UCC", "UCA", "UCG":
		protein = "Serine"
	case "UAU", "UAC":
		protein = "Tyrosine"
	case "UGU", "UGC":
		protein = "Cysteine"
	case "UGG":
		protein = "Tryptophan"
	case "UAA", "UAG", "UGA":
		err = ErrStop
	default:
		err = ErrInvalidBase
	}
	return
}

// FromRNA translates the rna strings to protein slices
func FromRNA(rna string) ([]string, error) {
	proteins := []string{}
	i := 0
	for i < len(rna) {
		codon := rna[i : i+3]
		protein, err := FromCodon(codon)
		if err == ErrStop {
			break
		}
		if err != nil {
			return proteins, err
		}
		proteins = append(proteins, protein)
		i += 3
	}
	return proteins, nil
}
