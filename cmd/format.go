/*
Copyright © 2021 NAME HERE <EMAIL ADDRESS>

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
	"encoding/json"
	"fmt"
	"github.com/UK-IPOP/drug-extraction/pkg"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"os"
	"strconv"
	"strings"
)

// formatCmd represents the format command
var formatCmd = &cobra.Command{
	Use:   "format",
	Short: "Formats output.jsonl into csv",
	Long: `This command transforms the 'output.jsonl' file into 
an 'output.csv' or 'output.json' file for consumption by external applications (spreadsheets/web-apis).`,
	Run: func(cmd *cobra.Command, args []string) {
		// this needs to be modularized and to accept args[0] as input sp can be called in pipeline
		convertFileData(args[0])
	},
}

var outputFormat string

func init() {
	rootCmd.AddCommand(formatCmd)
}

func convertFileData(newFileType string) {

	// run formatting
	// lets do json first quickly since its easier
	switch newFileType {
	case "json":
		// this loads the whole thing into memory which defeats the purpose of jsonlines
		// TODO: fix mentioned above
		old, err := os.OpenFile("data/output.jsonl", os.O_RDONLY, 0644)
		new, err := os.OpenFile("data/output.json", os.O_CREATE|os.O_WRONLY, 0644)
		pkg.Check(err)
		// read outputted jsonlines
		var results pkg.MultipleResults
		decoder := json.NewDecoder(old)
		for decoder.More() {
			// for each line
			var result pkg.Result
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
		old, err := os.OpenFile("data/output.jsonl", os.O_RDONLY, 0644)
		new, err := os.OpenFile("data/output.csv", os.O_CREATE|os.O_WRONLY, 0644)
		pkg.Check(err)
		// read outputted jsonlines
		headers := []string{"record_id", "drug_name", "word_found", "similarity_ratio", "tags"}
		new.WriteString(strings.Join(headers, ",") + "\n")
		decoder := json.NewDecoder(old)
		for decoder.More() {
			// for each line
			var result pkg.Result
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
