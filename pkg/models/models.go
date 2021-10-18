package models

import (
	_ "embed"
	"encoding/json"
	"gopkg.in/yaml.v2"
	"os"
)

// drugInfo contains the drug names, search terms, and tags in a configuration yaml file.
//go:embed drug_info.yaml
var drugInfo []byte

// Drug is the main data struct for parsing out each entry in the drugInfo yaml file.
type Drug struct {
	Name        string   `json:"name" yaml:"Name"`
	SearchTerms []string `json:"search_terms" yaml:"Search Terms"`
	Tags        []string `json:"tags" yaml:"Tags"`
}

// Drugs puts an array of the Drug items into one struct for easy serialization.
type Drugs struct {
	Drugs []Drug `json:"drugs" yaml:"Drugs"`
}

// Load reads the drug data from file and marshals it into Drugs.
func (Drugs) Load() Drugs {
	var drugs Drugs
	unmarshalErr := yaml.Unmarshal(drugInfo, &drugs)
	Check(unmarshalErr)
	return drugs
}

// Result is the struct to manage the output of the program.
// This constitutes the RecordID, DrugName, SimilarityRatio, WordFound and Tags labels which are
// all assembled when running search.
type Result struct {
	RecordID        string   `json:"record_id"`
	DrugName        string   `json:"drug_name"`
	SimilarityRatio float64  `json:"similarity_ratio"`
	WordFound       string   `json:"word_found"`
	Tags            []string `json:"tags"`
	TempID          int      `json:"-"` // ignores
}

// MultipleResults holds many Result items in an array for easy serialization.
type MultipleResults struct {
	Data []Result `json:"data"`
}

// ToFile writes MultipleResults to the specified filepath.
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

// TextSearchResult contains the specific results of search functionality and is used to create a full Result item.
type TextSearchResult struct {
	hasMatch        bool
	wordFound       string
	similarityRatio float64
}

// MatchResult contains information regarding whether an Exact or Close match was found.
type MatchResult struct {
	ExactMatch bool
	CloseMatch bool
}

// SearchResult contains information regarding the JaroWinkler Distance between the Source and the Target.
type SearchResult struct {
	Distance int // Jaro-Winkler distance
	Source   string
	Target   string
}
