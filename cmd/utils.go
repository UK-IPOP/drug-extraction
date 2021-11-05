package cmd

import (
	"errors"
	"fmt"
	"html/template"
	"math"
	"net/http"
	"os/exec"
	"path"
	"runtime"
	"sort"
	"strings"

	"github.com/UK-IPOP/drug-extraction/pkg/models"
)

// FindColIndex finds the index of the targeted column amongst the headers.
// Returns the integer index of the column.
func FindColIndex(headers []string, colName string) (int, error) {
	for i, col := range headers {
		cleanCol := strings.TrimSpace(col)
		if strings.EqualFold(cleanCol, strings.TrimSpace(colName)) {
			return i, nil
		}
	}
	return -1, errors.New("could not find column: " + colName)
}

// open opens the specified URL in the default browser of the user.
func open(url string) error {
	var cmd string
	var args []string

	switch runtime.GOOS {
	case "windows":
		cmd = "cmd"
		args = []string{"/c", "start"}
	case "darwin":
		cmd = "open"
	default: // "linux", "freebsd", "openbsd", "netbsd"
		cmd = "xdg-open"
	}
	args = append(args, url)
	return exec.Command(cmd, args...).Start()
}

type wordMatch struct {
	Word  string  `json:"word"`
	Ratio float64 `json:"ratio"`
}

type drugAnalysis struct {
	DrugName     string      `json:"drug_name"`
	MatchedWords []wordMatch `json:"matched_words"`
	TotalRecords int         `json:"total_records"`
}

type tagAnalysis struct {
	TagName string  `json:"tag_name"`
	Ratio   float64 `json:"ratio"`
}

func calculateDrugWordRatios(records []models.Result, threshold float64) []drugAnalysis {
	// get unique drugs
	uniqueDrugNames := map[string]bool{}
	for _, record := range records {
		uniqueDrugNames[record.DrugName] = true
	}

	// for each drug get its matched words and their ratios
	var drugData []drugAnalysis
	for drugName, _ := range uniqueDrugNames {
		var totalDrugRecords int
		drugWordCounts := map[string]int{}
		for _, record := range records {
			if record.DrugName == drugName {
				drugWordCounts[record.WordFound] += 1
				totalDrugRecords += 1
			}
		}
		var totalMatchesFound int
		for _, v := range drugWordCounts {
			totalMatchesFound += v
		}
		var matchedWords []wordMatch
		for k, v := range drugWordCounts {
			// v is count found, normalize using total count of words found related to this drug
			ratio := math.Round((float64(v)/float64(totalMatchesFound))*100) / 100
			if ratio > threshold {
				match := wordMatch{
					Word:  k,
					Ratio: ratio,
				}
				matchedWords = append(matchedWords, match)
			}
		}
		sort.Slice(matchedWords, func(i, j int) bool {
			return matchedWords[i].Ratio > matchedWords[j].Ratio
		})
		drugData = append(drugData, drugAnalysis{
			DrugName:     drugName,
			MatchedWords: matchedWords,
			TotalRecords: totalDrugRecords,
		})
	}

	return drugData
}

func calculateTags(records []models.Result) []tagAnalysis {
	totalRecords := len(records)
	var tagData []tagAnalysis
	tagCounts := map[string]int{}
	for _, record := range records {
		for _, tag := range record.Tags {
			tagCounts[tag] += 1
		}
	}
	for tag, count := range tagCounts {
		ratio := math.Round((float64(count))/float64(totalRecords)) * 100
		tagData = append(tagData, tagAnalysis{
			TagName: tag,
			Ratio:   ratio,
		})
	}
	return tagData
}

func runAnalysis(records models.MultipleResults, threshold float64) ([]drugAnalysis, []tagAnalysis) {
	analysis1 := calculateDrugWordRatios(records.Data, threshold)
	analysis2 := calculateTags(records.Data)
	return analysis1, analysis2
}

func reportHandler(w http.ResponseWriter, r *http.Request) {
	results := models.MultipleResults{}
	results.LoadFromFile("output.jsonl")
	pathPath := path.Join("web", "report.html")
	t, _ := template.ParseFiles(pathPath)
	drugAnalytics, tagAnalytics := runAnalysis(results, 0.01)
	analytics := make(map[string]interface{})
	analytics["drugs"] = drugAnalytics
	analytics["tags"] = tagAnalytics
	t.Execute(w, analytics)
	fmt.Println("report generated")
}
