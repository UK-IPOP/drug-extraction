import { Button } from "@nextui-org/react";
import * as React from "react";
import { SingleValue } from "react-select";
import DrugInput from "../drug_input";
import Limiter from "../limiter";
import SimpleInput from "../simple_input";
import { Drug, Phase3Options } from "../types";

interface Phase3Props {
    algorithm: string;
    searchType: string,
    dataHandler: (data: Phase3Options) => void

};


const Phase3Component = ({ algorithm, searchType, dataHandler }: Phase3Props): JSX.Element => {
    const [maxEdits, setMaxEdits] = React.useState<number>(0);
    const [minThresh, setMinThresh] = React.useState<number>(0.9);
    const [searchWords, setSearchWords] = React.useState<string[]>([]);
    const [drugList, setDrugList] = React.useState<Drug[]>([]);
    const [submitted, setSubmitted] = React.useState<boolean>(false);

    const handleDistanceSelect = (e: SingleValue<{ value: number; label: number; kind: string }>) => {
        if (e) {
            if (e.kind === 'distance') {
                setMaxEdits(e.value);
            } else {
                setMinThresh(e.value);
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
        <div>
            <h1>Phase 3</h1>

            <Limiter algorithm={algorithm} onSelected={handleDistanceSelect} />

            {/* // the search buttons and the continue button look VERY
            // similar. if they are still as similar in new UI,
            // make one of them different size/color. */}
            {searchType === "simple" &&
                <SimpleInput submitted={submitted} wordHandler={handleWords} />
            }

            {
                searchType === "drug" &&
                <DrugInput submitted={submitted} drugHandler={handleDrugInput} />
            }

            {submitted &&
                <Button onClick={() => dataHandler({ maxEdits, minThresh, searchWords, drugList })}>
                    Continue
                </Button>
            }

        </div >
    );
}

export default Phase3Component;