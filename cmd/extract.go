package cmd

import (
	"encoding/csv"
	"errors"
	"log"
	"os"
	"strings"

	"github.com/UK-IPOP/drug-extraction/pkg/models"

	"github.com/spf13/cobra"
)

// flag variables

// targetCol is the column to extract drugs from
var targetCol string

// idCol is the column to keep for later re-indexing/joining
var idCol string

// strictStatus is whether to perform strict-matching
var strictStatus bool

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
		fileName := args[0]
		idCol, idErr := cmd.Flags().GetString("id-col")
		targetCol, targetErr := cmd.Flags().GetString("target-col")
		if idErr != nil || targetErr != nil {
			log.Fatal("Missing required flags `--id-col` and `--target-col`")
		}
		ExtractServerRunner(
			fileName,
			idCol,
			targetCol,
			strictStatus,
		)
	},
}

func init() {
	rootCmd.AddCommand(extractCmd)

	// optional flags w/ defaults
	extractCmd.Flags().BoolVar(&strictStatus, "strict", false, "Whether to perform strict-matching")

	// required flags
	extractCmd.Flags().StringVar(&targetCol, "target-col", "", "Target column to extract drugs from")
	targetErr := extractCmd.MarkFlagRequired("target-col")
	models.Check(targetErr)
	extractCmd.Flags().StringVar(&idCol, "id-col", "", "ID column to keep for later re-indexing/joining")
	idErr := extractCmd.MarkFlagRequired("id-col")
	models.Check(idErr)
}

// ReadCsvFile reads the user-provided csv input file.
// Returns the headers of the file and the data contained in the file.
func ReadCsvFile(filePath string) ([]string, [][]string) {
	f, err := os.Open(filePath)
	if err != nil {
		log.Fatal("Unable to open input file "+filePath, err)
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
