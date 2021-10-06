package pkg

import (
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
