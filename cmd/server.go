package cmd

import (
	"net/http"
	"os"
	"path"
	"path/filepath"

	"github.com/UK-IPOP/drug-extraction/pkg/models"
	"github.com/fatih/color"
	"github.com/gin-gonic/gin"
	"github.com/spf13/cobra"
)

// serverCmd represents the report command
var serverCmd = &cobra.Command{
	Use:   "server",
	Short: "Server opens up an interactive web app",
	Long: `This command opens a web browser with interactivity
and a more user-friendly front-end interface for engaging with the
same background logic.`,
	Run: func(cmd *cobra.Command, args []string) {
		router := gin.Default()
		router.LoadHTMLGlob("./web/*")
		router.MaxMultipartMemory = 8 << 20 // 8 MiB

		router.GET("/", func(c *gin.Context) {
			c.HTML(http.StatusOK, "index.html", nil)
		})
		router.POST("/extract", func(c *gin.Context) {
			file, err := c.FormFile("formFile")
			if err != nil {
				c.HTML(http.StatusBadRequest, "error.html", err)
			}
			filename := filepath.Base(file.Filename)
			// Upload the file to specific dst.
			os.Mkdir("./uploads", 0777)
			filepath := path.Join("./uploads", filename)
			c.SaveUploadedFile(file, filepath)

			clean := c.PostForm("cleanStatus")
			if clean == "on" {
				CleanRunner()
			}

			strict := c.PostForm("strictStatus")
			var strictStatus bool
			if strict == "on" {
				strictStatus = true
			} else {
				strictStatus = false
			}

			// make this return error so we can go to error page
			ExtractServerRunner(filepath, c.PostForm("idCol"), c.PostForm("targetCol"), strictStatus)

			outputType := c.PostForm("inlineOutputOptions")
			if outputType != "jsonlines" {
				ConvertFileData(outputType)
			}

			c.HTML(http.StatusOK, "success.html", nil)
		})

		go open("http://localhost:8080")
		router.Run(":8080")
	},
}

func init() {
	rootCmd.AddCommand(serverCmd)
}

// ExtractServerRunner runs the extract command in server mode.
func ExtractServerRunner(fName string, idCol string, targetCol string, strictStatus bool) {
	fileName := fName
	headers, data := ReadCsvFile(fileName)
	idIndex, err1 := FindColIndex(headers, idCol)
	targetIndex, err2 := FindColIndex(headers, targetCol)
	models.Check(err1)
	models.Check(err2)

	color.Yellow("Using ID column -> %s (index=%v)", headers[idIndex], idIndex)
	color.Yellow("Using TextSearch column -> %s (index=%v)", headers[targetIndex], targetIndex)

	// actually process text
	var idData []string
	var targetData []string
	for _, row := range data {
		idData = append(idData, row[idIndex])
		targetData = append(targetData, row[targetIndex])
	}
	results := models.ScanDrugs(targetData, strictStatus)
	finalResults := models.MultipleResults{}
	for _, item := range results {
		id := idData[item.TempID] // row index lookup
		item.RecordID = id

		finalResults.Data = append(finalResults.Data, item)
	}

	// write to json
	finalResults.ToFile("output.jsonl")
}
