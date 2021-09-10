package pkg

import (
	"encoding/json"
	"io/ioutil"
	"os"
)

type Drug struct {
	Name        string   `json:"name"`
	SearchTerms []string `json:"search_terms"`
	Tags        []string `json:"tags"`
}

type Drugs struct {
	Drugs []Drug `json:"drugs"`
}

func (receiver Drugs) LoadFromFile() Drugs {
	// TODO: add in optional filepath param
	var drugs Drugs
	jsonFile, err := os.Open("./data/drug_classes.json")
	Check(err)
	defer func(jsonFile *os.File) {
		err := jsonFile.Close()
		Check(err)
	}(jsonFile)
	byteVal, err := ioutil.ReadAll(jsonFile)
	Check(err)
	unmarshalErr := json.Unmarshal(byteVal, &drugs)
	Check(unmarshalErr)
	return drugs
}
