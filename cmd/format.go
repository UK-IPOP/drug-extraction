package cmd

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/UK-IPOP/drug-extraction/pkg/models"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

// formatCmd represents the format command
var formatCmd = &cobra.Command{
	Use:   "format",
	Short: "Formats output.jsonl into csv",
	Long: `This command transforms the 'output.jsonl' file into 
an 'output.csv' or 'output.json' file for consumption by external applications (spreadsheets/web-apis).`,
	Run: func(cmd *cobra.Command, args []string) {
		ConvertFileData(args[0])
	},
}

func init() {
	rootCmd.AddCommand(formatCmd)
}

// ConvertFileData converts the ".jsonl" output to either ".json" or ".csv" output.
func ConvertFileData(newFileType string) error {

	// run formatting
	// lets do json first quickly since its easier
	switch newFileType {
	case "json":
		// TODO: this loads the whole thing into memory which defeats the purpose of jsonlines
		oldFile, err := os.OpenFile("output.jsonl", os.O_RDONLY, 0644)
		if err != nil {
			return err
		}
		newFile, err := os.OpenFile("output.json", os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return err
		}
		// read outputted jsonlines
		var results models.MultipleResults
		decoder := json.NewDecoder(oldFile)
		for decoder.More() {
			// for each line
			var result models.Result
			// parse into struct
			if err := decoder.Decode(&result); err != nil {
				fmt.Println("parse result: %w", err)
				return err
			}
			// append to struct
			results.Data = append(results.Data, result)
		}
		// write to file
		jsonResult, err := json.MarshalIndent(results, "", "    ")
		if err != nil {
			return err
		}
		_, err = newFile.Write(jsonResult)
		if err != nil {
			return err
		}
	case "csv":
		oldFile, err := os.OpenFile("output.jsonl", os.O_RDONLY, 0644)
		if err != nil {
			return err
		}
		newFile, err := os.OpenFile("output.csv", os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return err
		}
		// read outputted jsonlines
		headers := []string{"record_id", "drug_name", "word_found", "similarity_ratio", "tags"}
		_, err = newFile.WriteString(strings.Join(headers, ",") + "\n")
		if err != nil {
			return err
		}
		decoder := json.NewDecoder(oldFile)
		for decoder.More() {
			// for each line
			var result models.Result
			// parse into struct
			if err := decoder.Decode(&result); err != nil {
				fmt.Println("parse result: %w", err)
				return err
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
			if err != nil {
				return err
			}
		}
	default:
		color.Red("Unexpected file format, expected `csv` or `json`")
		return errors.New("unexpected file format, expected `csv` or `json`")
	}
	return nil
}
