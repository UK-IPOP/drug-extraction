package pkg

import (
	"encoding/json"
	"gopkg.in/yaml.v2"
	"io/ioutil"
)

type Drug struct {
	Name        string   `json:"name" yaml:"Name"`
	SearchTerms []string `json:"search_terms" yaml:"Search Terms"`
	Tags        []string `json:"tags" yaml:"Tags"`
}

type Drugs struct {
	Drugs []Drug `json:"drugs" yaml:"Drugs"`
}

func (Drugs) LoadFromFile() Drugs {
	// TODO: add in optional filepath param
	var drugs Drugs
	content, _ := ioutil.ReadFile("./data/drug_info.yaml")
	unmarshalErr := yaml.Unmarshal(content, &drugs)
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
