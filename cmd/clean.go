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
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"os"
)

// cleanCmd represents the clean command
var cleanCmd = &cobra.Command{
	Use:   "clean",
	Short: "Removes output files for clean runs.",
	Run: func(cmd *cobra.Command, args []string) {
		// TODO: add output files to remove, whatever they are
		files := []string{
			"data/output.json",
			"data/output.csv",
		}
		for _, file := range files {
			if _, err := os.Stat(file); err == nil {
				err = os.Remove(file)
				pkg.Check(err)
			}
		}
		color.Blue("Done removing old output files.")
	},
}

func init() {
	rootCmd.AddCommand(cleanCmd)
}
