import { Button, Col, Container, Grid, Link, Progress, Row, Spacer, Switch } from '@nextui-org/react';
import React from 'react';
import Phase1Component from './phases/phase1';
import Phase2Component from './phases/phase2';
import Phase3Component from './phases/phase3';
import Runner from './runner';
import { Phase2Options, Drug, Phase3Options } from './types';
import { Text } from "@nextui-org/react";

import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import PlayCircleOutlineIcon from '@mui/icons-material/PlayCircleOutline';
import RestartAltIcon from '@mui/icons-material/RestartAlt';

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));


const Interactive = (): JSX.Element => {
    const [fileData, setFileData] = React.useState<string[][]>([]);
    const [headers, setHeaders] = React.useState<string[]>([]);

    const [idColumnIndex, setIdColumnIndex] = React.useState<number>(-1);
    const [targetColumnIndex, setTargetColumnIndex] = React.useState<number>(-1);

    const [searchType, setSearchType] = React.useState<string>('simple');
    const [filterEdits, setFilterEdits] = React.useState<boolean>(true);

    const [maxEdits, setMaxEdits] = React.useState<number>(1);
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
        setFileData(data);
        setHeaders(headerRow);
        setPhase1(false);
        setPhase2(true);
        setProgress(30);
    };

    const handlePhase2 = (data: Phase2Options) => {
        setIdColumnIndex(data.idColumnIndex);
        setTargetColumnIndex(data.targetColumnIndex);
        setSearchType(data.searchType);
        setFilterEdits(data.filterType == 'edits' ? true : false);
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
            <Grid.Container justify="center">
                <Grid xs={8} justify="center">
                    <Progress status="primary" value={progress} />
                </Grid>
                <Grid xs={12} justify="center">
                    <Phase1Component dataHandler={handleFileData} />
                </Grid>
            </Grid.Container>
        )
    };

    if (phase2) {
        if (fileData.length == 0) {
            return (
                <Grid.Container justify="center">
                    <Grid xs={8} justify="center">
                        <Progress status="primary" value={progress} />
                    </Grid>
                    <Grid xs={12} justify="center">
                        <Link icon onClick={() => { setPhase1(true); setPhase2(false) }}><Text color="error">Please go back and input a file</Text></Link>
                        <Button icon onClick={() => { setPhase1(true); setPhase2(false) }} color="warning"><ArrowBackIcon /></Button>
                    </Grid>
                </Grid.Container>
            )
        }
        return (
            <Grid.Container justify="center">
                <Grid xs={8} justify="center">
                    <Progress status="primary" value={progress} />
                </Grid>
                <Grid xs={12} justify="center">
                    <Phase2Component headerOptions={headers} dataHandler={handlePhase2} />
                </Grid>
            </Grid.Container>
        )
    }
    if (phase3) {
        if (idColumnIndex < 0 || targetColumnIndex < 0) {
            return (
                <Grid.Container justify="center">
                    <Grid xs={8} justify="center">
                        <Progress status="primary" value={progress} />
                    </Grid>
                    <Grid xs={12} justify="center">
                        <Link icon onClick={() => { setPhase2(true); setPhase3(false) }}><Text color="error">Please select the columns containing the ID and target values.</Text></Link>
                    </Grid>
                    <Grid xs={12} justify="center">
                        <Button icon onClick={() => { setPhase2(true); setPhase3(false) }} color="warning"><ArrowBackIcon /></Button>
                    </Grid>
                </Grid.Container>
            )
        }
        return (
            <Grid.Container justify="center">
                <Grid xs={8} justify="center">
                    <Progress status="primary" value={progress} />
                </Grid>
                <Grid xs={12} justify="center">
                    <Phase3Component edits={filterEdits} searchType={searchType} dataHandler={handlePhase3} />
                </Grid>
            </Grid.Container>
        )
    }
    if (prepPhase) {
        return (
            <Grid.Container gap={2} justify="center">
                <Grid xs={8} justify="center">
                    <Progress status="primary" value={progress} />
                </Grid>
                <Grid xs={12} justify="center">
                    <Text h2>Selected Options</Text>
                </Grid>
                <Grid xs={12} justify="center">
                    <Text>Take this time to verify your options before continuing and running the program.</Text>
                </Grid>
                <Grid xs={12} justify="center">
                    <Text h5>ID Column: {headers[idColumnIndex]}</Text>
                </Grid>
                <Grid xs={12} justify="center">
                    <Text h5>Target Text Column: {headers[targetColumnIndex]}</Text>
                </Grid>

                <Grid xs={12} justify="center">
                    <Text h5>Search Type: {searchType}</Text>
                </Grid>
                <Grid xs={12} justify="center">
                    <Text h5>Limiter: {filterEdits ? "Edits <= " + maxEdits : "Similarity >=" + minThresh}</Text>
                </Grid>
                <Grid xs={12} justify="center">
                    {searchWords.length > 0 &&
                        <Text h5>Search Words: {searchWords.join(', ')}</Text>
                    }
                    {drugList.length > 0 &&
                        <Text h5>Drug List: {drugList.map(d => d.name).join(', ')}</Text>
                    }
                </Grid>
                <Grid xs={6} justify="flex-end">
                    <Button rounded onClick={() => { setPrepPhase(false); setPhase1(true) }}>
                        <RestartAltIcon /><Spacer />Restart</Button>
                </Grid>
                <Grid xs={6} justify="flex-start">
                    <Button rounded onClick={() => {
                        setProgress(100);
                        sleep(500).then(() => {
                            setPrepPhase(false);
                            setRunPhase(true);
                        });
                    }}>Run <Spacer /><PlayCircleOutlineIcon /></Button>
                </Grid>
            </Grid.Container >
        )
    }
    if (runPhase) {
        return (
            <Grid.Container justify="center">
                <Runner inputData={
                    {
                        idColumnIndex,
                        targetColumnIndex,
                        searchType,
                        maxEdits,
                        minThresh,
                        searchWords,
                        drugList,
                        headers,
                        data: fileData,
                    }
                } />
            </Grid.Container>
        )
    }
    return (
        <div>
            <h1>Interactive</h1>
            <p>This is an interactive version of the program...</p>
        </div>
    )
};

export default Interactive;
