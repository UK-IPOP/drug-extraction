package models

import (
	"fmt"
	"log"
	"strings"

	"github.com/adrg/strutil"
	"github.com/adrg/strutil/metrics"

	"github.com/schollz/progressbar/v3"
)

// SearchText takes a Drug and searches the specified text for any close matches.
// This uses the Jaro-Winkler distance (>= 0.90) to define a "close" match and distance == 1.0 for an "exact" match.
// SearchText returns a TextSearchResult instance.
func (d *Drug) SearchText(text string) TextSearchResult {
	for _, word := range strings.Fields(text) {
		for _, searchWord := range d.SearchTerms {
			lowerDrug := strings.ToLower(searchWord)
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

// SearchTextStrict does everything that SearchText does, but only looks for exact matches.
// Exact matches defined by Jaro-Winkler distance == 1.0. Returns a TextSearchResult instance.
func (d *Drug) SearchTextStrict(text string) TextSearchResult {
	for _, word := range strings.Fields(text) {
		for _, searchWord := range d.SearchTerms {
			lowerDrug := strings.ToLower(searchWord)
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

// initializeProgress initializes a pre-configured progress bar of a given length.
func initializeProgress(length int) *progressbar.ProgressBar {
	bar := progressbar.NewOptions(length,
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
	return bar
}

// ScanDrugs takes an array of text objects and scans each text instance for each potential Drug from the config file.
// Returns an array of Result objects.
func ScanDrugs(texts []string, strict bool) []Result {
	var results []Result
	drugList := Drugs{}.Load()
	bar := initializeProgress(len(texts))
	switch strict {
	case true:
		for i, text := range texts {
			t := strip(strings.ToLower(text))
			for _, drug := range drugList.Drugs {
				searchResult := drug.SearchTextStrict(t)
				if searchResult.hasMatch {
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
	case false:
		for i, text := range texts {
			t := strip(strings.ToLower(text))
			for _, drug := range drugList.Drugs {
				searchResult := drug.SearchText(t)
				if searchResult.hasMatch {
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
	default:
		log.Fatal("Invalid strict param provided")
		return results
	}
}
