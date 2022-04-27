import { Button, Progress, Switch } from '@nextui-org/react';
import React from 'react';
import Phase1Component from './phases/phase1';
import Phase2Component from './phases/phase2';
import Phase3Component from './phases/phase3';
import Runner from './runner';
import { Phase2Options, Drug, Phase3Options } from './types';


const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));


const Interactive = (): JSX.Element => {
    const [fileData, setFileData] = React.useState<string[][]>([]);
    const [headers, setHeaders] = React.useState<string[]>([]);

    const [idColumnIndex, setIdColumnIndex] = React.useState<number>(-1);
    const [targetColumnIndex, setTargetColumnIndex] = React.useState<number>(-1);
    const [algorithm, setAlgorithm] = React.useState<string>('Levenshtein');
    const [searchType, setSearchType] = React.useState<string>('simple');
    const [outputFormat, setOutputFormat] = React.useState<string>('CSV');
    const [analyze, setAnalyze] = React.useState<boolean>(false);

    const [maxEdits, setMaxEdits] = React.useState<number>(0);
    const [minThresh, setMinThresh] = React.useState<number>(0.9);
    const [searchWords, setSearchWords] = React.useState<string[]>([]);
    const [drugList, setDrugList] = React.useState<Drug[]>([]);

    const [phase1, setPhase1] = React.useState<boolean>(true);
    const [phase2, setPhase2] = React.useState<boolean>(false);
    const [phase3, setPhase3] = React.useState<boolean>(false);
    const [prepPhase, setPrepPhase] = React.useState<boolean>(false);
    const [runPhase, setRunPhase] = React.useState<boolean>(false);
    const [progress, setProgress] = React.useState<number>(0);

    const handleFileData = (data: string[][], headerRow: string[]) => {
        console.log(data[0]);
        console.log(headerRow);
        setFileData(data);
        setHeaders(headerRow);
        setPhase1(false);
        setPhase2(true);
        setProgress(30);
    };

    const handlePhase2 = (data: Phase2Options) => {
        setIdColumnIndex(data.idColumnIndex);
        setTargetColumnIndex(data.targetColumnIndex);
        setAlgorithm(data.algorithm);
        setSearchType(data.searchType);
        setOutputFormat(data.outputFormat);
        setAnalyze(data.analyze);
        setPhase2(false);
        setPhase3(true);
        setProgress(60);
    };

    const handlePhase3 = (data: Phase3Options) => {
        setMaxEdits(data.maxEdits ? data.maxEdits : 0);
        setMinThresh(data.minThresh ? data.minThresh : 0.9);
        setSearchWords(data.searchWords ? data.searchWords : []);
        setDrugList(data.drugList ? data.drugList : []);
        setPhase3(false);
        setPrepPhase(true);
        setProgress(90);
    };

    if (phase1) {
        return (
            <div>
                <Progress status="primary" value={progress} />
                <Phase1Component dataHandler={handleFileData} />
            </div>
        )
    };

    if (phase2) {
        return (
            <div>
                <Progress status="primary" value={progress} />
                <Phase2Component headerOptions={headers} dataHandler={handlePhase2} />
            </div>
        )
    }
    if (phase3) {
        return (
            <div>
                <Progress status="primary" value={progress} />
                <Phase3Component algorithm={algorithm} searchType={searchType} dataHandler={handlePhase3} />
            </div>
        )
    }
    if (prepPhase) {
        return (
            <div>
                {/* // TODO: CLEANUP this messaging */}
                <Progress status="primary" value={progress} />

                <h1>Options</h1>

                <p>You chose: {idColumnIndex > 0 ? headers[idColumnIndex] : "Nothing"} as your index column.</p>
                <p>You chose {headers[targetColumnIndex]} as your Target Column</p>

                <p>You chose {algorithm} as your algorithm</p>
                <p>You chose {outputFormat} as your output format</p>
                <p>You chose {analyze ? "Analyze" : "Don't Analyze"}</p>

                {/* // modify this based on algorithm to show only edits or thresh */}
                <p>You chose {maxEdits} as your max edits</p>
                <p>You chose {minThresh} as your min threshold</p>

                {/* // again should only show one based on mode */}
                {/* // for search words handle grammar if only one submitted (word vs words) */}
                <p>You chose {searchWords.length ? searchWords.join(', ') : "None"} as your search word(s)</p>
                <p>You chose {drugList.length ? drugList.map((x) => x.name).join(", ") : "None"} as your drugs</p>

                {/* // these should be next to each other */}
                <Button onClick={() => { setPrepPhase(false); setPhase1(true) }}>Restart</Button>
                <Button onClick={() => {
                    setProgress(100);
                    sleep(500).then(() => {
                        setPrepPhase(false);
                        setRunPhase(true);
                    });
                }}>Continue</Button>
            </div>
        )
    }
    if (runPhase) {
        return (
            <div>
                <Runner inputData={
                    {
                        idColumnIndex,
                        targetColumnIndex,
                        algorithm,
                        searchType,
                        outputFormat,
                        analyze,
                        maxEdits,
                        minThresh,
                        searchWords,
                        drugList,
                        headers,
                        data: fileData,
                    }
                } />
            </div>
        )
    }
    return (
        <div>
            <h1>Interactive</h1>
            <p>This is an interactive version of the program.  You can upload a file, and then choose the options to run the program.</p>
        </div>
    )
};

export default Interactive;
