/*
Copyright © 2021 Nick Anthony <nanthony007@gmail.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package cmd

import (
	"encoding/csv"
	"errors"
	"github.com/UK-IPOP/drug-extraction/pkg"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"log"
	"os"
	"strings"
)

// extractCmd represents the extract command
var extractCmd = &cobra.Command{
	Use:   "extract",
	Short: "Extract drugs",
	Long: `
Use this command to extract drugs from the specified file. 
For example:
	drug-extraction extract <your-filename.csv> --target-col text --id-col IDs

Data is expected in '*.csv' format.'`,
	Args: func(cmd *cobra.Command, args []string) error {
		// modularize this
		if len(args) != 1 {
			return errors.New("requires filename argument")
		}
		isCsv := strings.HasSuffix(args[0], ".csv")
		if !isCsv {
			return errors.New("file should be of type 'csv'")
		}
		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		// modularize this
		fileName := args[0]
		headers, data := readCsvFile(fileName)
		idFlag, _ := cmd.Flags().GetString("id-col")
		targetFlag, _ := cmd.Flags().GetString("target-col")
		idIndex, err1 := findColIndex(headers, idFlag)
		targetIndex, err2 := findColIndex(headers, targetFlag)
		pkg.Check(err1)
		pkg.Check(err2)
		// replace with nice prompt saying which cols ur using
		// add support for column case-ignoring (i.e. upper case)
		color.Yellow("Using ID column -> %s (index=%v)", headers[idIndex], idIndex)
		color.Yellow("Using TextSearch column -> %s (index=%v)", headers[targetIndex], targetIndex)

		// actually process text
		var idData []string
		var targetData []string
		for _, row := range data {
			idData = append(idData, row[idIndex])
			targetData = append(targetData, row[targetIndex])
		}
		results := pkg.ScanDrugs(targetData, strictStatus)
		finalResults := pkg.MultipleResults{}
		for _, item := range results {
			id := idData[item.TempID] // row index lookup
			item.RecordID = id

			finalResults.Data = append(finalResults.Data, item)
		}

		// write to json
		finalResults.ToFile("data/output.jsonl")

		// TODO: implement CSV later
		//fileHeaders := []string{idFlag, "DrugName", "MatchType", "WordFound", "Tags"}
		//writeCSV("data/output.csv", fileHeaders, finalResults)
	},
}

var targetCol string
var idCol string
var strictStatus bool

func init() {
	rootCmd.AddCommand(extractCmd)

	// optional flags w/ defaults
	extractCmd.Flags().BoolVar(&strictStatus, "strict", false, "Whether to perform strict-matching")

	// required flags
	extractCmd.Flags().StringVar(&targetCol, "target-col", "", "Target column to extract drugs from")
	targetErr := extractCmd.MarkFlagRequired("target-col")
	pkg.Check(targetErr)
	extractCmd.Flags().StringVar(&idCol, "id-col", "", "ID column to keep for later re-indexing/joining")
	idErr := extractCmd.MarkFlagRequired("id-col")
	pkg.Check(idErr)
}

func readCsvFile(filePath string) ([]string, [][]string) {
	f, err := os.Open(filePath)
	if err != nil {
		log.Fatal("Unable to read input file "+filePath, err)
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

// write csv func
func _(filePath string, headers []string, data [][]string) {
	file, err := os.Create(filePath)
	if err != nil {
		log.Fatal("Cannot create file", err)
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {

		}
	}(file)

	writer := csv.NewWriter(file)
	defer writer.Flush()

	writeErr := writer.Write(headers)
	pkg.Check(writeErr)
	for _, value := range data {
		err := writer.Write(value)
		pkg.Check(err)
	}
}

func findColIndex(headers []string, colName string) (int, error) {
	for i, col := range headers {
		if strings.ToLower(col) == strings.ToLower(colName) {
			return i, nil
		}
	}
	return -1, errors.New("could not find column: " + colName)
}