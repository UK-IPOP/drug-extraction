package main

import (
	"log"
)

func main() {
	scanner, fileErr := LoadFileStream()
	if fileErr != nil {
		log.Println("Unable to load data from file")
		log.Fatalln(fileErr)
	}
	algorithm, inputErr := GetUserInput()
	if inputErr != nil {
		log.Println("Unable to get user input")
		log.Fatalln(inputErr)
	}
	runErr := Runner(algorithm, scanner)
	if runErr != nil {
		log.Println("Unable to load data from file")
		log.Fatalln(runErr)
	}
}
