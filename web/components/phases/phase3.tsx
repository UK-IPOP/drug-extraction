import { Button, Container, FormElement, Spacer } from "@nextui-org/react";
import * as React from "react";
import DrugInput from "../drug_input";
import Limiter from "../limiter";
import SimpleInput from "../simple_input";
import { Text } from "@nextui-org/react";
import { Drug, Phase3Options } from "../types";
import styles from '../../styles/Home.module.css'
import { Link, Grid } from '@nextui-org/react';

interface Phase3Props {
    edits: boolean,
    searchType: string,
    dataHandler: (data: Phase3Options) => void

};


const Phase3Component = ({ edits, searchType, dataHandler }: Phase3Props): JSX.Element => {
    const [maxEdits, setMaxEdits] = React.useState<number>(1);
    const [minThresh, setMinThresh] = React.useState<number>(0.9);
    const [searchWords, setSearchWords] = React.useState<string[]>([]);
    const [drugList, setDrugList] = React.useState<Drug[]>([]);
    const [submitted, setSubmitted] = React.useState<boolean>(false);

    const handleDistanceSelect = (e: React.ChangeEvent<FormElement>) => {
        if (e && e.target && e.target.value) {
            if (edits) {
                const num = parseInt(e.target.value);
                setMaxEdits(num);
            } else {
                const num = parseFloat(e.target.value);
                setMinThresh(num);
            }
        }
    };
    const handleWords = (words: string[]) => {
        setSearchWords(words);
        setSubmitted(true);
    };

    const handleDrugInput = (drugs: Drug[]) => {
        setDrugList(drugs);
        setSubmitted(true);
    }

    return (
        <Grid.Container gap={2} justify="center">
            <Grid xs={12} justify="center">
                <Text h2 className={styles.subtitle}>Finally enter your search limiter and search terms:</Text>
            </Grid>

            <Grid xs={6} justify="center">
                <Limiter edits={edits} onSelected={handleDistanceSelect} />
            </Grid>

            <Grid xs={6} justify="center">
                {searchType === "simple" &&
                    <SimpleInput submitted={submitted} wordHandler={handleWords} />
                }
                {/* // renders one or the other */}
                {
                    searchType === "drug" &&
                    <DrugInput submitted={submitted} drugHandler={handleDrugInput} />
                }
            </Grid>

            <Grid xs={12} justify="center">
                {submitted &&
                    <Button
                        rounded
                        color="primary"
                        onClick={() => dataHandler({ maxEdits, minThresh, searchWords, drugList })}>
                        Continue
                    </Button>
                }
            </Grid>

        </Grid.Container >
    );
}

export default Phase3Component;