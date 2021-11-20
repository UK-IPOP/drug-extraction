package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"github.com/adrg/strutil"
	"github.com/adrg/strutil/metrics"
	"github.com/schollz/progressbar/v3"
	"log"
	"os"
	"strings"
	"time"
)

func main() {
	file, err := os.Open("../../data/records.jsonl")
	if err != nil {
		log.Fatalln(err)
	}
	defer file.Close()

	fmt.Println("Which algorithm do you want to run? (J/L)")
	var input string
	fmt.Scanln(&input)

	switch strings.TrimSpace(input) {
	case "L":
		runLevenshtein(file)
	case "J":
		runJaroWinkler(file)
	default:
		fmt.Println("neither ran")
	}
}

func runLevenshtein(file *os.File) {
	l := metrics.NewLevenshtein()
	scanner := bufio.NewScanner(file)
	outFile, err := os.Create("../../data/third-party/go-levenshtein.jsonl")
	if err != nil {
		log.Fatalln(err)
	}
	defer outFile.Close()
	bar := progressbar.Default(-1, "Searching text...")
	for scanner.Scan() {
		var record map[string]string
		json.Unmarshal(scanner.Bytes(), &record)
		record["primary_combined"] = joinCols(record)
		for _, level := range []string{"combined_primary", "secondarycause"} {
			if recordText, ok := record[level]; ok {
				results := searchRecordLevenshtein(recordText, level, l)
				if recordID, ok := record["casenumber"]; ok {
					for _, result := range results {
						fileData, err := json.Marshal(map[string]interface{}{
							"casenumber": recordID,
							"results":    result,
						})
						if err != nil {
							log.Fatalln(err)
						}
						_, err = outFile.Write(append(fileData, []byte("\n")...))
						if err != nil {
							fmt.Println("could not write to file")
							log.Fatalln(err)
						}
					}
				}
			}
		}
		bar.Add(1)
	}
}

func runJaroWinkler(file *os.File) {
	j := metrics.NewJaroWinkler()
	scanner := bufio.NewScanner(file)
	outFile, err := os.Create("../../data/third-party/go-jaro.jsonl")
	if err != nil {
		log.Fatalln(err)
	}
	defer outFile.Close()
	bar := progressbar.Default(-1, "Searching text...")
	for scanner.Scan() {
		var record map[string]string
		json.Unmarshal(scanner.Bytes(), &record)
		record["primary_combined"] = joinCols(record)
		for _, level := range []string{"combined_primary", "secondarycause"} {
			if recordText, ok := record[level]; ok {
				results := searchRecordJaroWinkler(recordText, level, j)
				if recordID, ok := record["casenumber"]; ok {
					for _, result := range results {
						fileData, err := json.Marshal(map[string]interface{}{
							"casenumber": recordID,
							"results":    result,
						})
						if err != nil {
							log.Fatalln(err)
						}
						_, err = outFile.Write(append(fileData, []byte("\n")...))
						if err != nil {
							fmt.Println("could not write to file")
							log.Fatalln(err)
						}
					}
				}
			}
		}
		bar.Add(1)
	}
}

// joins column values
func joinCols(record map[string]string) string {
	var result string
	one, ok1 := record["primarycause"]
	two, ok2 := record["primarycause_linea"]
	three, ok3 := record["primarycause_lineb"]
	four, ok4 := record["primarycause_linec"]
	if ok1 {
		result += one
	}
	if ok2 {
		result += two
	}
	if ok3 {
		result += three
	}
	if ok4 {
		result += four
	}
	return result
}

// makes alpha num
func strip(s string) string {
	var result strings.Builder
	for i := 0; i < len(s); i++ {
		b := s[i]
		if ('a' <= b && b <= 'z') ||
			('A' <= b && b <= 'Z') ||
			('0' <= b && b <= '9') ||
			b == ' ' {
			result.WriteByte(b)
		}
	}
	return result.String()
}

func searchRecordLevenshtein(text string, level string, searcher *metrics.Levenshtein) []map[string]interface{} {
	y := strip(text)
	var data []map[string]interface{}
	for _, word := range strings.Split(y, " ") {
		sTime := time.Time{}
		data = append(data,
			map[string]interface{}{
			"word": word,
			"distance": strutil.Similarity(word, "heroin", searcher),
			"level": level,
			"metric": "Normalized Levenshtein",
			"time": time.Since(sTime),
		})
	}
	return data
}

func searchRecordJaroWinkler(text string, level string, searcher *metrics.JaroWinkler) []map[string]interface{} {
	y := strip(text)
	var data []map[string]interface{}
	for _, word := range strings.Split(y, " ") {
		sTime := time.Time{}
		data = append(data,
			map[string]interface{}{
				"word": word,
				"distance": strutil.Similarity(word, "heroin", searcher),
				"level": level,
				"metric": "Jaro-Winkler",
				"time": time.Since(sTime),
			})
	}
	return data
}