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
	"github.com/UK-IPOP/drug-extraction/pkg"

	"github.com/spf13/cobra"
)

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
			convertFileData(formatType)
		}
	},
}

var cleanStatus bool
var formatStatus bool
var formatType string

func init() {
	rootCmd.AddCommand(pipelineCmd)

	// not required flags with defaults
	pipelineCmd.Flags().BoolVar(&cleanStatus, "clean", false, "Remove existing output files for a clean run")
	pipelineCmd.Flags().BoolVar(&strictStatus, "strict", false, "Whether to perform strict-matching")
	pipelineCmd.Flags().BoolVar(&formatStatus, "format", false, "Whether to format the data")
	pipelineCmd.Flags().StringVar(&formatType, "format-type", "csv", "The new data format")

	// required file flags for extraction
	pipelineCmd.Flags().StringVar(&targetCol, "target-col", "", "Target column to extract drugs from")
	targetErr := pipelineCmd.MarkFlagRequired("target-col")
	pkg.Check(targetErr)
	pipelineCmd.Flags().StringVar(&idCol, "id-col", "", "ID column to keep for later re-indexing/joining")
	idErr := pipelineCmd.MarkFlagRequired("id-col")
	pkg.Check(idErr)
}