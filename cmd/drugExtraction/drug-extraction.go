package main

import (
	"fmt"
	"github.com/UK-IPOP/drug-extraction/pkg"
)

func main() {
	fmt.Println("hi there")
	drugList := pkg.Drugs{}.LoadFromFile()
	for _, drug := range drugList.Drugs {
		fmt.Println(drug.Name)
	}
}
