package cmd

import (
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
