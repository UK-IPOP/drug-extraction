package models

import (
	"encoding/json"
	"io"
	"io/ioutil"
	"log"
	"strings"
)

// Check quickly handles the checking of errors and logs if the error is not nil.
func Check(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

// strip removes punctuation from the string provided.
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

func readJsonLines(fpath string) []Result {
	contents, err := ioutil.ReadFile(fpath)
	Check(err)
	dec := json.NewDecoder(strings.NewReader(string(contents)))
	var results []Result
	for {
		var result Result
		err := dec.Decode(&result)
		if err != nil {
			if err == io.EOF {
				break
			}
			log.Fatal(err)
		}
		results = append(results, result)
	}
	return results
}
