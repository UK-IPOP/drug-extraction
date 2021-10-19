package cmd

import (
	"github.com/UK-IPOP/drug-extraction/pkg/models"
	"github.com/spf13/cobra"
)

// flag variables

// cleanStatus is a flag for whether to clean the output files
var cleanStatus bool

// formatStatus is a flag for whether to format the data
var formatStatus bool

// formatType is a flag for the new data format
var formatType string

// pipelineCmd represents the pipeline command
var pipelineCmd = &cobra.Command{
	Use:   "pipeline",
	Short: "Runs entire command pipeline",
	Long: `This command runs various subcommands.
It will always call the 'extract' command and thus inherits its required flags.
It can optionally call the 'clean' command prior to 'extract' if the '--clean' flag is provided.`,
	Run: func(cmd *cobra.Command, args []string) {
		if cleanStatus {
			cleanCmd.Run(cmd, args)
		}
		extractCmd.Run(cmd, args)
		if formatStatus {
			ConvertFileData(formatType)
		}
	},
}

func init() {
	rootCmd.AddCommand(pipelineCmd)

	// not required flags with defaults
	pipelineCmd.Flags().BoolVar(&cleanStatus, "clean", true, "Remove existing output files for a clean run")
	pipelineCmd.Flags().BoolVar(&strictStatus, "strict", false, "Whether to perform strict-matching")
	pipelineCmd.Flags().BoolVar(&formatStatus, "format", false, "Whether to format the data")
	pipelineCmd.Flags().StringVar(&formatType, "format-type", "csv", "The new data format")

	// required file flags for extraction
	pipelineCmd.Flags().StringVar(&targetCol, "target-col", "", "Target column to extract drugs from")
	targetErr := pipelineCmd.MarkFlagRequired("target-col")
	models.Check(targetErr)
	pipelineCmd.Flags().StringVar(&idCol, "id-col", "", "ID column to keep for later re-indexing/joining")
	idErr := pipelineCmd.MarkFlagRequired("id-col")
	models.Check(idErr)
}
