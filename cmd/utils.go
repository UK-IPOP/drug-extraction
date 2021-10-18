package cmd

import (
	"encoding/csv"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/UK-IPOP/drug-extraction/pkg/models"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"log"
	"os"
	"path"
	"strconv"
	"strings"
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

// ConvertFileData converts the ".jsonl" output to either ".json" or ".csv" output.
func ConvertFileData(newFileType string) {

	// run formatting
	// lets do json first quickly since its easier
	switch newFileType {
	case "json":
		// this loads the whole thing into memory which defeats the purpose of jsonlines
		// TODO: fix mentioned above
		old, err := os.OpenFile("output.jsonl", os.O_RDONLY, 0644)
		new, err := os.OpenFile("output.json", os.O_CREATE|os.O_WRONLY, 0644)
		models.Check(err)
		// read outputted jsonlines
		var results models.MultipleResults
		decoder := json.NewDecoder(old)
		for decoder.More() {
			// for each line
			var result models.Result
			// parse into struct
			if err := decoder.Decode(&result); err != nil {
				fmt.Errorf("parse result: %w", err)
			}
			// append to struct
			results.Data = append(results.Data, result)
		}
		// write to file
		jsonResult, _ := json.MarshalIndent(results, "", "    ")
		new.Write(jsonResult)
	case "csv":
		old, err := os.OpenFile("output.jsonl", os.O_RDONLY, 0644)
		new, err := os.OpenFile("output.csv", os.O_CREATE|os.O_WRONLY, 0644)
		models.Check(err)
		// read outputted jsonlines
		headers := []string{"record_id", "drug_name", "word_found", "similarity_ratio", "tags"}
		new.WriteString(strings.Join(headers, ",") + "\n")
		decoder := json.NewDecoder(old)
		for decoder.More() {
			// for each line
			var result models.Result
			// parse into struct
			if err := decoder.Decode(&result); err != nil {
				fmt.Errorf("parse result: %w", err)
			}
			// write to file
			var row = make([]string, 5)
			row[0] = result.RecordID
			row[1] = result.DrugName
			row[2] = result.WordFound
			row[3] = strconv.FormatFloat(result.SimilarityRatio, 'f', -1, 64)
			row[4] = strings.Join(result.Tags, ";")
			rowString := strings.Join(row, ",")
			new.WriteString(rowString + "\n")
		}
	default:
		color.Red("Unexpected file format, expected `csv` or `json`")

	}
}
