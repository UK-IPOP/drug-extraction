import { Button, Grid, Loading, Text } from "@nextui-org/react";
import Link from "next/link";
import * as React from "react";
import { AlgorithmInput, AlgorithmOutputDrug, AlgorithmOutputSimple } from "./types";
import Execute from "./main_program";

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

interface RunnerProps {
    inputData: AlgorithmInput;
};


const Runner = ({ inputData }: RunnerProps): JSX.Element => {
    const [completed, setCompleted] = React.useState<boolean>(false);

    console.log("running execute")
    const results = Execute(inputData)

    console.log("results", results.length);

    postFile(results).then(json => {
        console.log(json);
        setCompleted(true);
    });


    if (completed) {
        return (
            <Grid.Container gap={2} justify="center">
                <Grid xs={12} justify="center">
                    <Text h1 color="primary">Done!</Text>
                </Grid>
                <Grid xs={6} justify="flex-end">
                    <Link href="/" passHref={true}>
                        <Button rounded size="lg" bordered borderWeight="light">Home</Button>
                    </Link>
                </Grid>
                <Grid xs={6} justify="flex-start">
                    <Button size="lg"
                        type="submit"
                        bordered
                        borderWeight="light"
                        rounded
                        onClick={() => {
                            fetch("/api/extract")
                                .then(res => res.blob())
                                .then(blob => {
                                    // hacky but i don't care
                                    const url = window.URL.createObjectURL(blob);
                                    const a = document.createElement("a");
                                    a.href = url;
                                    a.download = "extracted_drugs.csv";
                                    a.click();
                                })
                                .finally(() => {
                                    window.location.href = "/finished";
                                });
                        }}
                    > Download results
                    </Button>
                </Grid>
            </Grid.Container >
        )
    }
    return (
        <Grid.Container justify="center">
            <Grid justify="center">
                <Text h2>Running... </Text>
                <Loading type="points" />
            </Grid>
        </Grid.Container >
    )
}

const postFile = async (data: AlgorithmOutputSimple[] | AlgorithmOutputDrug[]) => {
    const response = await fetch("/api/dump-to-file", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    if (response.status !== 201) {
        throw new Error("Failed to post file");
    }
    const json = await response.json();
    return json;
}

export default Runner;