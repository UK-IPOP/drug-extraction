package cmd

import (
	"encoding/csv"
	"encoding/json"
	"errors"
	"fmt"
	"html/template"
	"log"
	"math"
	"net/http"
	"os"
	"os/exec"
	"path"
	"runtime"
	"sort"
	"strconv"
	"strings"

	"github.com/UK-IPOP/drug-extraction/pkg/models"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

// CleanRunner executes on the 'clean' command, it removes all the files specified.
func CleanRunner() {
	files := []string{
		"output.json",
		"output.csv",
		"output.jsonl",
	}
	for _, file := range files {
		fpath := path.Join("data", file)
		if _, err := os.Stat(fpath); err == nil {
			err = os.Remove(fpath)
			models.Check(err)
		}
	}
	color.Blue("Removed old output files.")
}

// ReadCsvFile reads the user-provided csv input file.
// Returns the headers of the file and the data contained in the file.
func ReadCsvFile(filePath string) ([]string, [][]string) {
	f, err := os.Open(filePath)
	if err != nil {
		log.Fatal("Unable to open input file "+filePath, err)
	}
	defer func(f *os.File) {
		err := f.Close()
		if err != nil {

		}
	}(f)

	csvReader := csv.NewReader(f)
	records, err := csvReader.ReadAll()
	if err != nil {
		log.Fatal("Unable to parse file as CSV for "+filePath, err)
	}
	headers := records[0]
	data := records[1:]
	return headers, data
}

// FindColIndex finds the index of the targeted column amongst the headers.
// Returns the integer index of the column.
func FindColIndex(headers []string, colName string) (int, error) {
	for i, col := range headers {
		if strings.ToLower(col) == strings.ToLower(colName) {
			return i, nil
		}
	}
	return -1, errors.New("could not find column: " + colName)
}

// ExtractRunner executes on the `extract` command.
// This function reads the specified csv file, and checks the target-column for Drug
// instances using ScanDrugs.
func ExtractRunner(cmd *cobra.Command, fName string, strictStatus bool) {
	fileName := fName
	headers, data := ReadCsvFile(fileName)
	idFlag, _ := cmd.Flags().GetString("id-col")
	targetFlag, _ := cmd.Flags().GetString("target-col")
	idIndex, err1 := FindColIndex(headers, idFlag)
	targetIndex, err2 := FindColIndex(headers, targetFlag)
	models.Check(err1)
	models.Check(err2)

	color.Yellow("Using ID column -> %s (index=%v)", headers[idIndex], idIndex)
	color.Yellow("Using TextSearch column -> %s (index=%v)", headers[targetIndex], targetIndex)

	// actually process text
	var idData []string
	var targetData []string
	for _, row := range data {
		idData = append(idData, row[idIndex])
		targetData = append(targetData, row[targetIndex])
	}
	results := models.ScanDrugs(targetData, strictStatus)
	finalResults := models.MultipleResults{}
	for _, item := range results {
		id := idData[item.TempID] // row index lookup
		item.RecordID = id

		finalResults.Data = append(finalResults.Data, item)
	}

	// write to json
	finalResults.ToFile("output.jsonl")
}

func ExtractServerRunner(fName string, idCol string, targetCol string, strictStatus bool) {
	fileName := fName
	headers, data := ReadCsvFile(fileName)
	idIndex, err1 := FindColIndex(headers, idCol)
	targetIndex, err2 := FindColIndex(headers, targetCol)
	models.Check(err1)
	models.Check(err2)

	color.Yellow("Using ID column -> %s (index=%v)", headers[idIndex], idIndex)
	color.Yellow("Using TextSearch column -> %s (index=%v)", headers[targetIndex], targetIndex)

	// actually process text
	var idData []string
	var targetData []string
	for _, row := range data {
		idData = append(idData, row[idIndex])
		targetData = append(targetData, row[targetIndex])
	}
	results := models.ScanDrugs(targetData, strictStatus)
	finalResults := models.MultipleResults{}
	for _, item := range results {
		id := idData[item.TempID] // row index lookup
		item.RecordID = id

		finalResults.Data = append(finalResults.Data, item)
	}

	// write to json
	finalResults.ToFile("output.jsonl")
}


// ConvertFileData converts the ".jsonl" output to either ".json" or ".csv" output.
func ConvertFileData(newFileType string) {

	// run formatting
	// lets do json first quickly since its easier
	switch newFileType {
	case "json":
		// this loads the whole thing into memory which defeats the purpose of jsonlines
		// TODO: fix mentioned above
		oldFile, err := os.OpenFile("output.jsonl", os.O_RDONLY, 0644)
		newFile, err := os.OpenFile("output.json", os.O_CREATE|os.O_WRONLY, 0644)
		models.Check(err)
		// read outputted jsonlines
		var results models.MultipleResults
		decoder := json.NewDecoder(oldFile)
		for decoder.More() {
			// for each line
			var result models.Result
			// parse into struct
			if err := decoder.Decode(&result); err != nil {
				fmt.Println("parse result: %w", err)
			}
			// append to struct
			results.Data = append(results.Data, result)
		}
		// write to file
		jsonResult, _ := json.MarshalIndent(results, "", "    ")
		_, err = newFile.Write(jsonResult)
		models.Check(err)
	case "csv":
		oldFile, err := os.OpenFile("output.jsonl", os.O_RDONLY, 0644)
		newFile, err := os.OpenFile("output.csv", os.O_CREATE|os.O_WRONLY, 0644)
		models.Check(err)
		// read outputted jsonlines
		headers := []string{"record_id", "drug_name", "word_found", "similarity_ratio", "tags"}
		_, err = newFile.WriteString(strings.Join(headers, ",") + "\n")
		models.Check(err)
		decoder := json.NewDecoder(oldFile)
		for decoder.More() {
			// for each line
			var result models.Result
			// parse into struct
			if err := decoder.Decode(&result); err != nil {
				fmt.Println("parse result: %w", err)
			}
			// write to file
			var row = make([]string, 5)
			row[0] = result.RecordID
			row[1] = result.DrugName
			row[2] = result.WordFound
			row[3] = strconv.FormatFloat(result.SimilarityRatio, 'f', -1, 64)
			row[4] = strings.Join(result.Tags, ";")
			rowString := strings.Join(row, ",")
			_, err = newFile.WriteString(rowString + "\n")
			models.Check(err)
		}
	default:
		color.Red("Unexpected file format, expected `csv` or `json`")

	}
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
