// Package main runs the CLI.
package main

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
)

type AutoGenerated struct {
	UserInput struct {
		ClassID    string   `json:"classId"`
		Trans      string   `json:"trans"`
		RelaSource string   `json:"relaSource"`
		Rela       string   `json:"rela"`
		Ttys       []string `json:"ttys"`
	} `json:"userInput"`
	DrugMemberGroup struct {
		DrugMember []struct {
			MinConcept struct {
				Rxcui string `json:"rxcui"`
				Name  string `json:"name"`
				Tty   string `json:"tty"`
			} `json:"minConcept"`
			NodeAttr []struct {
				AttrName  string `json:"attrName"`
				AttrValue string `json:"attrValue"`
			} `json:"nodeAttr"`
		} `json:"drugMember"`
	} `json:"drugMemberGroup"`
}

type Drug struct {
	Rxcui string `json:"rxcui"`
	Name  string `json:"name"`
	Group string `json:"group"`
}

func runner() {
	response, err := http.Get("https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json?classId=D009294&relaSource=MESH")
	if err != nil {
		log.Println(err)
	}
	body, err := ioutil.ReadAll(response.Body)
	if err != nil {
		log.Fatalln(err)
	}
	var rawData AutoGenerated
	err = json.Unmarshal(body, &rawData)
	if err != nil {
		log.Fatal(err)
	}
	var data []Drug
	for _, drugMember := range rawData.DrugMemberGroup.DrugMember {
		data = append(data, Drug{Rxcui: drugMember.MinConcept.Rxcui, Name: drugMember.MinConcept.Name, Group: "Narcotic"})
	}

	// next
	r1, err2 := http.Get("https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json?classId=N02A&relaSource=ATC")
	if err2 != nil {
		log.Println(err2)
	}
	body2, err3 := ioutil.ReadAll(r1.Body)
	if err3 != nil {
		log.Fatalln(err3)
	}
	var rawData2 AutoGenerated
	json.Unmarshal(body2, &rawData2)
	for _, drugMember := range rawData2.DrugMemberGroup.DrugMember {
		data = append(data, Drug{Rxcui: drugMember.MinConcept.Rxcui, Name: drugMember.MinConcept.Name, Group: "Opioid"})
	}
	log.Println(data)
}
