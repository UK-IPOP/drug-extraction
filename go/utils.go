package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"math"
	"os"
	"strings"
	"time"

	"github.com/adrg/strutil/metrics"
)

func LoadFileStream() (*bufio.Scanner, error) {
	file, fileErr := os.Open("../data/records.jsonl")
	if fileErr != nil {
		log.Fatalln("could not open file", fileErr)
		return nil, fileErr
	}
	scanner := bufio.NewScanner(file)
	return scanner, nil
}

func GetUserInput() (string, error) {
	var choice string
	fmt.Println("Which algorithm do you want to run?")
	fmt.Print("JaroWinkler or Levenshtein? (J/L): ")
	_, err := fmt.Scan(&choice)
	if err != nil {
		return "", err
	}
	if choice == "J" || choice == "L" {
		return choice, nil
	} else {
		return "", fmt.Errorf("expected 'J' or 'L', got %s", choice)
	}
}

// clean makes alpha num
func clean(s string) string {
	text := strings.ToUpper(s)
	text = strings.ReplaceAll(text, "(", "")
	text = strings.ReplaceAll(text, ")", "")
	text = strings.ReplaceAll(text, ",", "")
	text = strings.ReplaceAll(text, ";", "")
	text = strings.ReplaceAll(text, ":", "")
	text = strings.ReplaceAll(text, "@", "")
	text = strings.ReplaceAll(text, "#", "")
	text = strings.ReplaceAll(text, "$", "")
	text = strings.ReplaceAll(text, "%", "")
	text = strings.ReplaceAll(text, "^", "")
	text = strings.ReplaceAll(text, "&", "")
	text = strings.ReplaceAll(text, "*", "")
	text = strings.ReplaceAll(text, "_", "")
	text = strings.ReplaceAll(text, "+", "")
	text = strings.ReplaceAll(text, "=", "")
	text = strings.ReplaceAll(text, "{", "")
	text = strings.ReplaceAll(text, "}", "")
	text = strings.ReplaceAll(text, "[", "")
	text = strings.ReplaceAll(text, "]", "")
	text = strings.ReplaceAll(text, "|", "")
	text = strings.ReplaceAll(text, "<", "")
	text = strings.ReplaceAll(text, ">", "")
	text = strings.ReplaceAll(text, "/", "")
	return text
}

// joins column values
func joinCols(record map[string]interface{}) string {
	var result string
	one, ok1 := record["primarycause"]
	two, ok2 := record["primarycause_linea"]
	three, ok3 := record["primarycause_lineb"]
	four, ok4 := record["primarycause_linec"]
	if ok1 {
		result += fmt.Sprintf("%v", one)
	}
	if ok2 {
		result += fmt.Sprintf("%v", two)
	}
	if ok3 {
		result += fmt.Sprintf("%v", three)
	}
	if ok4 {
		result += fmt.Sprintf("%v", four)
	}
	return result
}

func searchRecord(text string, level string, searchType string) []map[string]interface{} {
	var data []map[string]interface{}
	cleanText := strip(text)
	switch searchType {
	case "L":
		searcher := metrics.NewLevenshtein()
		for _, word := range strings.Fields(cleanText) {
			if word == "" {
				continue
			}
			sTime := time.Now()
			d := searcher.Distance(word, "heroin")
			eTime := time.Since(sTime).Seconds()
			distance := 1 - (float64(d) / math.Max(float64(len(word)), float64(len("heroin"))))
			data = append(data,
				map[string]interface{}{
					"word":     word,
					"distance": distance,
					"level":    level,
					"metric":   "NormalizedLevenshtein",
					"time":     eTime,
				})
		}
	case "J":
		searcher := metrics.NewJaroWinkler()
		for _, word := range strings.Fields(cleanText) {
			if word == "" {
				continue
			}
			sTime := time.Now()
			distance := searcher.Compare(word, "heroin")
			eTime := time.Since(sTime).Seconds()
			data = append(data,
				map[string]interface{}{
					"word":     word,
					"distance": distance,
					"level":    level,
					"metric":   "JaroWinkler",
					"time":     eTime,
				})
		}
	}
	return data
}

func Runner(searchMetric string, fileData *bufio.Scanner) error {
	// prepare search params
	var fpathEnding string
	switch searchMetric {
	case "J":
		fpathEnding = "go-jarowinkler"
	case "L":
		fpathEnding = "go-levenshtein"
	default:
		return errors.New("invalid search metric")
	}

	outFilePath := fmt.Sprintf("../data/%s.jsonl", fpathEnding)
	outFile, outFileCreationErr := os.Create(outFilePath)
	if outFileCreationErr != nil {
		log.Fatalln(outFileCreationErr)
		return outFileCreationErr
	}
	defer func(outFile *os.File) {
		err := outFile.Close()
		if err != nil {
			log.Fatalln("could not close output file", err)
		}
	}(outFile)

	for fileData.Scan() {
		var record map[string]interface{}
		jsonErr := json.Unmarshal(fileData.Bytes(), &record)
		if jsonErr != nil {
			return jsonErr
		}
		record["primary_combined"] = joinCols(record)
		for _, col := range []string{"primary_combined", "secondarycause"} {
			if recordText, ok := record[col]; ok {
				searchResults := searchRecord(fmt.Sprintf("%s", recordText), col, searchMetric)
				if len(searchResults) == 0 {
					continue
				}
				if recordID, ok2 := record["casenumber"]; ok2 {
					outData, jsonMarshalErr := json.Marshal(map[string]interface{}{
						"casenumber": recordID,
						"results":    searchResults,
					})
					if jsonMarshalErr != nil {
						return jsonMarshalErr
					}
					_, outFileWriteErr := outFile.Write(append(outData, []byte("\n")...))
					if outFileWriteErr != nil {
						return outFileWriteErr
					}
				}
			}
		}
	}
	return nil
}
