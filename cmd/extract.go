package cmd

import (
	"errors"
	"github.com/UK-IPOP/drug-extraction/pkg/models"
	"strings"

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
		ExtractRunner(cmd, args[0], strictStatus)
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
