import { Card, FormElement, Grid, Input, Link, Spacer, Text } from "@nextui-org/react";
import * as React from "react";
import { Drug } from "./types";
import Button from '@mui/material/Button';

interface DrugInputProps {
    submitted: boolean;
    drugHandler: (drugs: Drug[],) => void
};



/// probably should throw user error if this fails
const fetchDrugs = async (classId: string, relationship: string): Promise<Drug[]> => {
    const response = await fetch(`https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json?classId=${classId}&relaSource=${relationship}`);
    const json = await response.json();
    const drugResponse: Root = JSON.parse(JSON.stringify(json));
    const drugs = drugResponse.drugMemberGroup.drugMember.map(member => {
        const drug: Drug = {
            name: member.minConcept.name,
            rxID: member.minConcept.rxcui,
            classID: classId,
        };
        return drug;
    });
    return drugs;
}


const DrugInput = ({ submitted, drugHandler }: DrugInputProps): JSX.Element => {
    const [drugCode, setDrugCode] = React.useState<string>("");
    const [relaSource, setRelaSource] = React.useState<string>("");
    const [disabled, setDisabled] = React.useState<boolean>(false);
    const [label1, setLabel1] = React.useState<string>("Drug Search:");
    const [label2, setLabel2] = React.useState<string>("Drug RelaSource:");

    const handleDrugCode = (e: React.ChangeEvent<FormElement>) => {
        setDrugCode(e.target.value);
    }

    const handleRelaSource = (e: React.ChangeEvent<FormElement>) => {
        setRelaSource(e.target.value);
    }

    const handleSubmit = async () => {
        const drugList = await fetchDrugs(drugCode, relaSource);
        setDisabled(true)
        setLabel1("Drug Search (locked):")
        setLabel2("Drug RelaSource (locked):")
        drugHandler(drugList);
    }

    return (
        <Card>
            <Text h5>Consult <Link href="https://mor.nlm.nih.gov/RxNav/" target="_blank" icon>RxNav</Link> or our <Link href="https://github.com/UK-IPOP/drug-extraction" target="_blank" icon>Documentation</Link> for help.</Text>
            <Input color="primary" readOnly={disabled} label={label1} placeholder="RxCUI ID" onChange={handleDrugCode} helperText="RxCUI from RxNorm" />
            <Spacer y={2} />
            <Input color="primary" readOnly={disabled} label={label2} placeholder="RxClass RelaSource" onChange={handleRelaSource} helperText="RelaSource from RxClass (usually either ATC or MESH)" />
            <Spacer y={2} />
            {!submitted && <Button variant="outlined" color="success" onClick={handleSubmit}>Fetch Drugs</Button>}
        </Card>
    )
}


export default DrugInput;


export interface Root {
    drugMemberGroup: DrugMemberGroup
}

export interface DrugMemberGroup {
    drugMember: DrugMember[]
}

export interface DrugMember {
    minConcept: MinConcept
    nodeAttr: NodeAttr[]
}

export interface MinConcept {
    rxcui: string
    name: string
    tty: string
}

export interface NodeAttr {
    attrName: string
    attrValue: string
}
