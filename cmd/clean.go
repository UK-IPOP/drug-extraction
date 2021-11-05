package cmd

import (
	"os"
	"path"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

// cleanCmd represents the clean command
var cleanCmd = &cobra.Command{
	Use:   "clean",
	Short: "Removes output files for clean runs.",
	Run: func(cmd *cobra.Command, args []string) {
		CleanRunner()
	},
}

func init() {
	rootCmd.AddCommand(cleanCmd)
}

// CleanRunner executes on the 'clean' command, it removes all the files specified.
func CleanRunner() error {
	e, err := os.Executable()
	if err != nil {
		return err
	}
	files := []string{
		"output.json",
		"output.csv",
		"output.jsonl",
	}
	for _, file := range files {
		fpathDir := path.Join(path.Dir(e), file)
		if _, err := os.Stat(fpathDir); err == nil {
			err = os.Remove(fpathDir)
			if err != nil {
				return err
			}
		}
	}
	color.Blue("Removed old output files.")
	return nil
}
