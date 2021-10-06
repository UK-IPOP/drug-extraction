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
	"fmt"
	"github.com/UK-IPOP/drug-extraction/pkg"
	"github.com/spf13/cobra"
	"log"
	"os"
	"strconv"
	"strings"
)

var fileName string
var targetCol string
var idCol string

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
		fmt.Println("extract called")
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
		fmt.Println(idIndex, targetIndex)

		// actually process text
		var idData []string
		var targetData []string
		for _, row := range data {
			idData = append(idData, row[idIndex])
			targetData = append(targetData, row[targetIndex])
		}
		results := pkg.ScanDrugs(targetData)
		// the first value of targetData is the row you can lookup in the idData
		// TODO: this data structuring can be improved
		var idResults [][]string
		for _, item := range results {
			index, _ := strconv.Atoi(item[0]) // row index to lookup
			id := idData[index]
			// directly replace row-lookup w/ actual ID
			// should be changed into a struct!
			item[0] = id
			// item = append(item, id)  // add ID as last value
			idResults = append(idResults, item)
		}
		fmt.Println(idResults)
		// now write to file
		fileHeaders := []string{idFlag, "DrugName", "MatchType", "WordFound", "Tags"}
		writeCSV("data/output.csv", fileHeaders, idResults)
	},
}

func init() {
	rootCmd.AddCommand(extractCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports local flags which will only run when this command is called directly, e.g.:
	// extractCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	extractCmd.Flags().StringVar(&targetCol, "target-col", "", "Target column to extract drugs from")
	extractCmd.MarkFlagRequired("target-col")
	extractCmd.Flags().StringVar(&idCol, "id-col", "", "ID column to keep for later re-indexing/joining")
	extractCmd.MarkFlagRequired("id-col")
}

func readCsvFile(filePath string) ([]string, [][]string) {
	f, err := os.Open(filePath)
	if err != nil {
		log.Fatal("Unable to read input file "+filePath, err)
	}
	defer f.Close()

	csvReader := csv.NewReader(f)
	records, err := csvReader.ReadAll()
	if err != nil {
		log.Fatal("Unable to parse file as CSV for "+filePath, err)
	}
	headers := records[0]
	data := records[1:]
	return headers, data
}

func writeCSV(filePath string, headers []string, data [][]string) {
	// headers := []string{"ID", "DrugName", "MatchType", "Tags"}
	file, err := os.Create(filePath)
	if err != nil {
		log.Fatal("Cannot create file", err)
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	writer.Write(headers)
	for _, value := range data {
		err := writer.Write(value)
		pkg.Check(err)
	}
}

func findColIndex(headers []string, colName string) (int, error) {
	for i, col := range headers {
		if col == colName {
			return i, nil
		}
	}
	return -1, errors.New("could not find column: " + colName)
}
