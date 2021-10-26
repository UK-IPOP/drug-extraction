package cmd

import (
	"fmt"
	"net/http"
	"os"
	"path"
	"path/filepath"

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
		router.MaxMultipartMemory = 8 << 20  // 8 MiB

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

			// clean := c.PostForm("cleanStatus")
			// var cleanStatus bool
			// if clean =="on" { 
			// 	strictStatus = true
			// } else {
			// 	strictStatus = false
			// }

			// TODO: add strict status field
			// TODO: add clean runner

			// CleanRunner()
			

			ExtractServerRunner(filepath, c.PostForm("idCol"), c.PostForm("targetCol"), false)

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

func fileSubmitHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20) // limit your max input length!

	_, header, err := r.FormFile("file")
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println("further")
	headers, _ := ReadCsvFile(header.Filename)
	fmt.Println("further")
	fmt.Println(headers)

}
