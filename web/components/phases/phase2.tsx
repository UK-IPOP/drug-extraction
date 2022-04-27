import { Button, Switch } from "@nextui-org/react";
import * as React from "react";
import Selector from "../selector";
import { Phase2Options } from "../types";

interface Phase2Props {
    headerOptions: string[];
    dataHandler: (data: Phase2Options) => void

};


// TODO: fill in the placeholders with their corresponding defaults


const Phase2Component = ({ headerOptions, dataHandler }: Phase2Props): JSX.Element => {
    const [idColumn, setIdColumn] = React.useState<number>(-1);
    const [targetCol, setTargetCol] = React.useState<number>(-1);
    const [algorithm, setAlgorithm] = React.useState<string>('Levenshtein');
    const [searchType, setSearchType] = React.useState<string>('simple');
    const [outputFormat, setOutputFormat] = React.useState<string>('dense');
    const [analyze, setAnalyze] = React.useState<boolean>(false);

    const handleOutputFormatSelect = (e: { value: number; label: string }) => {
        setOutputFormat(e.label);
    };
    const handleIDSelect = (e: { value: number; label: string }) => {
        if (e) {
            setIdColumn(e.value);
        } else {
            setIdColumn(null);
        }
    };
    const handleTargetSelect = (e: { value: number; label: string }) => {
        setTargetCol(e.value);
    };
    const handleAlgorithmSelect = (e: { value: number; label: string }) => {
        setAlgorithm(e.label);
    };
    return (
        <div>
            <h1>Phase 2</h1>

            <label>ID Column: (leave blank for no ID)</label>
            <Selector
                optionsList={headerOptions}
                placeholder="Select an ID column"
                onSelected={handleIDSelect}
                clearable={true}
            />

            <label>Target Column:</label>
            <Selector
                optionsList={headerOptions}
                placeholder="Select a Target column"
                onSelected={handleTargetSelect}
            />

            <p>More algorithms coming soon... ('Damerau', 'OSA', 'JaroWinkler', 'SorensenDice')</p>
            <label>Algorithm: (recommended Levenshtein)</label>
            <Selector
                optionsList={['Levenshtein',]}
                placeholder="Select an Algorithm"
                onSelected={handleAlgorithmSelect}
            />

            <label>Search Type: (simple/custom or RxNorm-based Drug)</label>
            <Selector
                optionsList={['simple', 'drug']}
                placeholder="Select a Search Type"
                onSelected={(e) => setSearchType(e.label)}
            />


            <label>Output Format: (wide = record-level, dense = counts)</label>
            <Selector
                optionsList={['dense', 'wide']}
                placeholder="Select an Output Format"
                onSelected={handleOutputFormatSelect}
            />

            <label>Would you like some brief analytical quips?</label>
            <Switch onChange={(x) => setAnalyze(x.target.checked)} />

            <Button onClick={() => {
                dataHandler({
                    idColumnIndex: idColumn,
                    targetColumnIndex: targetCol,
                    algorithm: algorithm,
                    searchType: searchType,
                    outputFormat: outputFormat,
                    analyze: analyze
                });
            }}>Continue</Button>
        </div>
    );

};

export default Phase2Component;