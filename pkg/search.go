package pkg

import (
	"fmt"
	"strings"

	"github.com/adrg/strutil"
	"github.com/adrg/strutil/metrics"

	"github.com/schollz/progressbar/v3"
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
	for _, word := range strings.Fields(text) {
		for _, searchWord := range d.SearchTerms {
			lowerDrug := strings.ToLower(searchWord)
			// get similarity ratio
			// use jaro-winkler distance to identify typos
			sim := strutil.Similarity(lowerDrug, word, metrics.NewJaroWinkler())
			if sim >= 0.90 {
				return TextSearchResult{
					hasMatch:        true,
					wordFound:       word,
					similarityRatio: sim,
				}
			}
		}
	}
	return TextSearchResult{
		hasMatch:        false,
		wordFound:       "",
		similarityRatio: 0.0,
	}
}

func (d *Drug) SearchTextStrict(text string) TextSearchResult {
	for _, word := range strings.Fields(text) {
		for _, searchWord := range d.SearchTerms {
			lowerDrug := strings.ToLower(searchWord)
			// get similarity ratio
			// use jaro-winkler distance to identify typos
			sim := strutil.Similarity(lowerDrug, word, metrics.NewJaroWinkler())
			if sim == 1.0 {
				return TextSearchResult{
					hasMatch:        true,
					wordFound:       word,
					similarityRatio: sim,
				}
			}
		}
	}
	return TextSearchResult{
		hasMatch:        false,
		wordFound:       "",
		similarityRatio: 0.0,
	}
}

func ScanDrugs(texts []string, strict bool) []Result {
	var results []Result
	drugList := Drugs{}.Load()
	bar := progressbar.NewOptions(len(texts),
		progressbar.OptionEnableColorCodes(true),
		progressbar.OptionSetWidth(20),
		progressbar.OptionSetDescription("[blue]Extracting drugs...[reset] "),
		progressbar.OptionSetTheme(progressbar.Theme{
			Saucer:        "[green]=[reset]",
			SaucerHead:    "[green]>[reset]",
			SaucerPadding: " ",
			BarStart:      "[",
			BarEnd:        "]",
		}))
	if strict {
		for i, text := range texts {
			t := strip(strings.ToLower(text))

			for _, drug := range drugList.Drugs {
				searchResult := drug.SearchTextStrict(t)
				if searchResult.hasMatch {
					// found something so now add it
					// make this string array into a struct
					r := Result{
						DrugName:        drug.Name,
						SimilarityRatio: searchResult.similarityRatio,
						WordFound:       searchResult.wordFound,
						Tags:            drug.Tags,
						TempID:          i,
					}
					results = append(results, r)
				}
			}
			err := bar.Add(1)
			Check(err)
		}
		fmt.Println()
		return results
	} else {
		for i, text := range texts {
			t := strip(strings.ToLower(text))

			for _, drug := range drugList.Drugs {
				searchResult := drug.SearchText(t)
				if searchResult.hasMatch {
					// found something so now add it
					// make this string array into a struct
					r := Result{
						DrugName:        drug.Name,
						SimilarityRatio: searchResult.similarityRatio,
						WordFound:       searchResult.wordFound,
						Tags:            drug.Tags,
						TempID:          i,
					}
					results = append(results, r)
				}
			}
			err := bar.Add(1)
			Check(err)
		}
		fmt.Println()
		return results
	}
}
