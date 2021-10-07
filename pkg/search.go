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

func (d *Drug) SearchText(text string) TextSearchResult {
	//TODO: can modify to return a struct for safety
	for _, word := range strings.Fields(text) {
		for _, searchWord := range d.SearchTerms {
			lowerWord := strings.ToLower(word)
			lowerDrug := strings.ToLower(searchWord)
			// search for exact match
			if lowerDrug == lowerWord {
				return TextSearchResult{
					hasMatch:  true,
					wordFound: lowerWord,
					matchType: "Exact",
				}
			} else {
				// search for close match
				distance := fuzzy.LevenshteinDistance(lowerWord, lowerDrug)
				var maxDistance int
				if len(lowerDrug) > len(lowerWord) {
					maxDistance = len(lowerDrug)
				} else {
					maxDistance = len(lowerWord)
				}
				var distanceRatio = float64(distance) / float64(maxDistance)
				if absRatio := math.Abs(distanceRatio); absRatio <= 0.20 {
					return TextSearchResult{
						hasMatch:  true,
						wordFound: lowerWord,
						matchType: "Close",
					}
				}
			}
		}
	}
	return TextSearchResult{
		hasMatch:  false,
		wordFound: "",
		matchType: "",
	}
}

func ScanDrugs(texts []string) []Result {
	var results []Result
	drugList := Drugs{}.LoadFromFile()
	for i, row := range texts {
		for _, drug := range drugList.Drugs {
			searchResult := drug.SearchText(row)
			if searchResult.hasMatch {
				// found something so now add it
				// make this string array into a struct
				r := Result{
					DrugName:  drug.Name,
					MatchType: searchResult.matchType,
					WordFound: searchResult.wordFound,
					Tags:      drug.Tags,
					TempID:    i,
				}
				results = append(results, r)
			}
		}
	}
	return results
}
