package pkg

import (
	"github.com/lithammer/fuzzysearch/fuzzy"
	"math"
	"strings"
)

type MatchResult struct {
	ExactMatch bool
	CloseMatch bool
}

type SearchResult struct {
	Distance int
	Source   string
	Target   string
}

func (d *Drug) containsExactMatch(text string) (bool, string) {
	// would like this to take a single string OR array of strings
	for _, word := range strings.Fields(text) {
		for _, searchWord := range d.SearchTerms {
			if strings.ToLower(searchWord) == strings.ToLower(word) {
				return true, strings.ToLower(word)
			}
		}
	}
	return false, ""
}

func (d *Drug) containsCloseMatch(text string) (bool, string) {
	words := strings.Fields(text)
	for _, word := range words {
		for _, searchWord := range d.SearchTerms {
			lowerWord := strings.ToLower(word)
			lowerDrug := strings.ToLower(searchWord)
			// might need custom implementation of distance
			// would need to rely on ratio -> actual distance / max distance gives range 0-1
			distance := fuzzy.LevenshteinDistance(lowerWord, lowerDrug)
			var maxDistance int
			if len(lowerDrug) > len(lowerWord) {
				maxDistance = len(lowerDrug)
			} else {
				maxDistance = len(lowerWord)
			}
			var distanceRatio = float64(distance) / float64(maxDistance)
			if absRatio := math.Abs(distanceRatio); absRatio <= 0.20 {
				return true, lowerWord
			}
		}
	}
	return false, ""
}

func (d *Drug) SearchText(text string) (string, string) {
	// returns match TYPE
	// TODO: can modify to return an enum for safety
	exactMatch, exactWord := d.containsExactMatch(text)
	closeMatch, closeWord := d.containsCloseMatch(text)
	if exactMatch {
		return "Exact", exactWord
	} else if closeMatch {
		return "Close", closeWord
	} else {
		return "", ""
	}
}

func ScanDrugs(texts []string) []Result {
	var results []Result
	drugList := Drugs{}.LoadFromFile()
	for i, row := range texts {
		for _, drug := range drugList.Drugs {
			resultType, resultWord := drug.SearchText(row)
			if resultType != "" {
				// found something so now add it
				// make this string array into a struct
				r := Result{
					DrugName:  drug.Name,
					MatchType: resultType,
					WordFound: resultWord,
					Tags:      drug.Tags,
					TempID:    i,
				}
				results = append(results, r)
			}
		}
	}
	return results
}
