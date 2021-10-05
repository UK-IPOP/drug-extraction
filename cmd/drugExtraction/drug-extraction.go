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
}
