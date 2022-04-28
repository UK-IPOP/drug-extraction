import { Button, Grid, Loading, Spacer, Text } from "@nextui-org/react";
import Link from "next/link";
import * as React from "react";
import { AlgorithmInput, AlgorithmOutputDrug, AlgorithmOutputSimple } from "./types";
import Execute from "./main_program";
import CloudDownloadIcon from '@mui/icons-material/CloudDownload';
import HomeIcon from '@mui/icons-material/Home';

interface RunnerProps {
    inputData: AlgorithmInput;
};


const Runner = ({ inputData }: RunnerProps): JSX.Element => {
    const [completed, setCompleted] = React.useState<boolean>(false);

    console.log("running execute")
    const results = Execute(inputData)

    postFile(results).then(res => {
        console.log(res)
        setCompleted(true);
    })

    console.log("results", results.length);

    if (completed) {
        return (
            <Grid.Container gap={2} justify="center">
                <Grid xs={12} justify="center">
                    <Text h1 color="primary">Done!</Text>
                </Grid>
                <Grid xs={6} justify="flex-end">
                    <Link href="/" passHref={true}>
                        <Button rounded size="lg" bordered borderWeight="light">
                            <HomeIcon /><Spacer />Home</Button>
                    </Link>
                </Grid>
                <Grid xs={6} justify="flex-start">
                    <Button size="lg"
                        type="submit"
                        bordered
                        borderWeight="light"
                        rounded
                        onClick={() => {
                            fetch("/api/results")
                                .then(res => res.blob())
                                .then(blob => {
                                    // hacky but i don't care
                                    const url = window.URL.createObjectURL(blob);
                                    const a = document.createElement("a");
                                    a.href = url;
                                    a.download = "results.csv";
                                    a.click();
                                })
                                .finally(() => {
                                    window.location.href = "/finished";
                                });
                        }}
                    > Download results <Spacer /> <CloudDownloadIcon /> </Button>
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
    await fetch("/api/extract", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
}

export default Runner;