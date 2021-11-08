package main

import (
	"encoding/json"
	"io/ioutil"

	"github.com/UK-IPOP/drug-extraction/pkg/models"
)

func main() {
	drugs := models.Drugs{}.Load()
	file, _ := json.MarshalIndent(drugs, "", " ")

	_ = ioutil.WriteFile("search_terms.json", file, 0644)

}
