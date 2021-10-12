package pkg

import (
	_ "embed"
	"encoding/json"
	"gopkg.in/yaml.v2"
	"os"
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
	RecordID        string   `json:"record_id"`
	DrugName        string   `json:"drug_name"`
	SimilarityRatio float64  `json:"similarity_ratio"`
	WordFound       string   `json:"word_found"`
	Tags            []string `json:"tags"`
	TempID          int      `json:"-"` // ignores it when writing to json
}

type MultipleResults struct {
	Data []Result `json:"data"`
}

func (r MultipleResults) ToFile(path string) {
	f, err := os.OpenFile(path, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	for _, info := range r.Data {
		jsonified, _ := json.Marshal(info)
		if _, err = f.Write(append(jsonified, "\n"...)); err != nil {
			panic(err)
		}
	}
}

type TextSearchResult struct {
	hasMatch        bool
	wordFound       string
	similarityRatio float64 // should be either "Close" or "Exact" or ""... find a way to enforce?
}
