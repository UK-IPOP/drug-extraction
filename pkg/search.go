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

func (d *Drug) containsExactMatch(text string) bool {
	// would like this to take a single string OR array of strings
	for _, word := range strings.Fields(text) {
		if strings.ToLower(d.Name) == strings.ToLower(word) {
			return true
		}
	}
	return false
}

func (d *Drug) containsCloseMatch(text string) bool {
	words := strings.Fields(text)
	for _, word := range words {
		lowerWord := strings.ToLower(word)
		lowerDrug := strings.ToLower(d.Name)
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
			return true
		}
	}
	return false
}

func (d *Drug) SearchText(text string) MatchResult {
	exactMatch := d.containsExactMatch(text)
	closeMatch := d.containsCloseMatch(text)
	var matchResult MatchResult
	matchResult.ExactMatch = exactMatch
	matchResult.CloseMatch = closeMatch
	return matchResult
}

//func ScanDrugs(texts []string) []string {
//	var results [][]string
//	drugList := Drugs{}.LoadFromFile()
//	for i, row := range texts {
//		for _, drug := range drugList.Drugs {
//			searchResults := drug.SearchText(row)
//			if searchResults.CloseMatch || searchResults.ExactMatch {
//				stringCloseMatch := strconv.FormatBool(searchResults.CloseMatch)
//				stringExactMatch := strconv.FormatBool(searchResults.ExactMatch)
//				results = append(results, []string{drug.Name, stringExactMatch, stringCloseMatch})
//			}
//		}
//	}
//
//	return results
//}
