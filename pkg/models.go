package pkg

import (
	_ "embed"
	"encoding/json"
	"gopkg.in/yaml.v2"
	"io/ioutil"
)

//go:embed drug_info.yaml
var drugInfo []byte

type Drug struct {
	Name        string   `json:"name" yaml:"Name"`
	SearchTerms []string `json:"search_terms" yaml:"Search Terms"`
	Tags        []string `json:"tags" yaml:"Tags"`
}

type Drugs struct {
	Drugs []Drug `json:"drugs" yaml:"Drugs"`
}

func (Drugs) Load() Drugs {
	var drugs Drugs
	unmarshalErr := yaml.Unmarshal(drugInfo, &drugs)
	Check(unmarshalErr)
	return drugs
}

type Result struct {
	RecordID  string   `json:"record_id"`
	DrugName  string   `json:"drug_name"`
	MatchType string   `json:"match_type"`
	WordFound string   `json:"word_found"`
	Tags      []string `json:"tags"`
	TempID    int      `json:"-"` // ignores it when writing to json
}

type FileResult struct {
	Data []Result `json:"data"`
}

func (r FileResult) ToFile(path string) {
	jsonData, convertErr := json.MarshalIndent(r, "", "  ")
	Check(convertErr)
	writeErr := ioutil.WriteFile(path, jsonData, 0644)
	Check(writeErr)
}

type TextSearchResult struct {
	hasMatch  bool
	wordFound string
	matchType string // should be either "Close" or "Exact" or ""... find a way to enforce?
}
