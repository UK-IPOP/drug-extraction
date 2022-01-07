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
	file, fileErr := os.Open("../data/input/records.jsonl")
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
		result += fmt.Sprintf(" %s", one)
	}
	if ok2 {
		result += fmt.Sprintf(" %s", two)
	}
	if ok3 {
		result += fmt.Sprintf(" %s", three)
	}
	if ok4 {
		result += fmt.Sprintf(" %s", four)
	}
	return strings.TrimSpace(result)
}

func loadDrugs() ([]map[string]string, error) {
	file, fileErr := os.Open("../data/input/drugs.jsonl")
	if fileErr != nil {
		log.Fatalln("could not open drug file", fileErr)
		return nil, fileErr
	}
	defer file.Close()
	var drugs []map[string]string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var drug map[string]string
		err := json.Unmarshal(scanner.Bytes(), &drug)
		if err != nil {
			log.Fatalln("could not unmarshal drug", err)
			return nil, err
		}
		drugs = append(drugs, drug)
	}
	return drugs, nil
}

func searchRecord(text string, level string, searchType string, drugList []map[string]string) []map[string]interface{} {
	var data []map[string]interface{}
	cleanText := clean(text)
	switch searchType {
	case "L":
		searcher := metrics.NewLevenshtein()
		for _, drug := range drugList {
			drugWords := strings.Split(drug["name"], "/")
			drugId := drug["rx_id"]
			var drugNames []string
			for _, drugName := range drugWords {
				drugNames = append(drugNames, strings.TrimSpace(strings.ToUpper(drugName)))
			}
			for _, name := range drugNames {
				for _, word := range strings.Fields(cleanText) {
					if word == "" {
						continue
					}
					sTime := time.Now()
					d := searcher.Distance(word, name)
					eTime := time.Since(sTime).Seconds()
					distance := 1 - (float64(d) / math.Max(float64(len(word)), float64(len(name))))
					data = append(data,
						map[string]interface{}{
							"word":       word,
							"similarity": distance,
							"level":      level,
							"metric":     "NormalizedLevenshtein",
							"time":       eTime,
							"drug":       name,
							"drug_id":    drugId,
						})
				}
			}
		}
	case "J":
		searcher := metrics.NewJaroWinkler()
		for _, drug := range drugList {
			drugWords := strings.Split(drug["name"], "/")
			drugId := drug["rx_id"]
			var drugNames []string
			for _, drugName := range drugWords {
				drugNames = append(drugNames, strings.TrimSpace(strings.ToUpper(drugName)))
			}
			for _, name := range drugNames {
				for _, word := range strings.Fields(cleanText) {
					if word == "" {
						continue
					}
					sTime := time.Now()
					d := searcher.Compare(word, name)
					eTime := time.Since(sTime).Seconds()
					distance := 1 - (float64(d) / math.Max(float64(len(word)), float64(len(name))))
					data = append(data,
						map[string]interface{}{
							"word":       word,
							"similarity": distance,
							"level":      level,
							"metric":     "JaroWinkler",
							"time":       eTime,
							"drug":       name,
							"drug_id":    drugId,
						})
				}
			}
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

	drugs, drugLoadErr := loadDrugs()
	if drugLoadErr != nil {
		log.Fatalln("could not load drugs", drugLoadErr)
		return drugLoadErr
	}
	outFilePath := fmt.Sprintf("../data/output/%s.jsonl", fpathEnding)
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
				searchResults := searchRecord(fmt.Sprintf("%s", recordText), col, searchMetric, drugs)
				if len(searchResults) == 0 {
					continue
				}
				if recordID, ok2 := record["casenumber"]; ok2 {
					for _, result := range searchResults {
						result["casenumber"] = recordID
						outData, jsonMarshalErr := json.Marshal(result)
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
	}
	return nil
}
