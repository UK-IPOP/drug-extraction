package main

import (
	"fmt"
	"github.com/UK-IPOP/drug-extraction/pkg"
)

func main() {
	var drugList = pkg.Drugs{}.LoadFromFile()
	text := "i think cocaine and alcoho are really fun!"
	for _, drug := range drugList.Drugs {
		result := drug.SearchText(text)
		if result.CloseMatch || result.ExactMatch {
			fmt.Println(drug.Name, result)
		}
	}
	// this was used to convert json to yaml once
	//var drugs pkg.Drugs
	//jsonFile, err := os.Open("./data/drug_classes.json")
	//pkg.Check(err)
	//defer func(jsonFile *os.File) {
	//	err := jsonFile.Close()
	//	pkg.Check(err)
	//}(jsonFile)
	//byteVal, err := ioutil.ReadAll(jsonFile)
	//pkg.Check(err)
	//unmarshalErr := json.Unmarshal(byteVal, &drugs)
	//pkg.Check(unmarshalErr)
	//fmt.Println("loaded json")
	//drugYaml, _ := yaml.Marshal(&drugs)
	//ioutil.WriteFile("./data/drug_info.yaml", drugYaml, 0644)
}
