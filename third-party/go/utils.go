package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"math"
	"os"
	"strings"
	"time"

	"github.com/adrg/strutil/metrics"
	"github.com/schollz/progressbar/v3"
)

func lineCounter(r io.Reader) (int, error) {
	buf := make([]byte, 32*1024)
	count := 0
	lineSep := []byte{'\n'}
	for {
		c, err := r.Read(buf)
		count += bytes.Count(buf[:c], lineSep)
		switch {
		case err == io.EOF:
			return count, nil
		case err != nil:
			return count, err
		}
	}
}

func LoadFileStream() (*bufio.Scanner, int, error) {
	file, fileErr := os.Open("../../data/records.jsonl")
	if fileErr != nil {
		log.Fatalln("could not open file", fileErr)
		return nil, -1, fileErr
	}
	lines, lineCountErr := lineCounter(file)
	if lineCountErr != nil {
		log.Fatalln("could not count lines in input file")
		return nil, -1, lineCountErr
	}
	file2, fileErr2 := os.Open("../../data/records.jsonl")
	if fileErr != nil {
		log.Fatalln("could not open file", fileErr2)
		return nil, -1, fileErr2
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			log.Fatalln("could not close input file", err)
		}
	}(file)
	scanner := bufio.NewScanner(file2)
	return scanner, lines, nil
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
		return "", errors.New(fmt.Sprintf("expected 'J' or 'L', got %s", choice))
	}
}

// strip makes alpha num
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
					"metric":   "Normalized Levenshtein",
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

func initializeProgress(length int) *progressbar.ProgressBar {
	bar := progressbar.NewOptions(length,
		progressbar.OptionEnableColorCodes(true),
		progressbar.OptionSetWidth(20),
		progressbar.OptionSetDescription("[blue]Comparing strings...[reset] "),
		progressbar.OptionSetTheme(progressbar.Theme{
			Saucer:        "[green]=[reset]",
			SaucerHead:    "[green]>[reset]",
			SaucerPadding: " ",
			BarStart:      "[",
			BarEnd:        "]",
		}))
	return bar
}

func Runner(searchMetric string, fileData *bufio.Scanner, fileLines int) error {
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

	outFilePath := fmt.Sprintf("../../data/third-party/%s.jsonl", fpathEnding)
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

	bar := initializeProgress(fileLines)
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
		progressBarErr := bar.Add(1)
		if progressBarErr != nil {
			return progressBarErr
		}
	}
	return nil
}
