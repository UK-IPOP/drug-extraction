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
    const [output, setOutput] = React.useState<string>("");

    console.log("running execute");
    const results = Execute(inputData);
    console.log("results", results.length);

    postFile(results).then((res) => {
        setCompleted(true);
        setOutput(res);
    });

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
                            download("results.csv", output);

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
            </Grid>
            <Grid justify="center">
                <Loading type="points" />
            </Grid>
        </Grid.Container >
    )
}

const postFile = async (data: AlgorithmOutputSimple[] | AlgorithmOutputDrug[]): Promise<string> => {
    const response = await fetch("/api/extract", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    return response.text();
}

// makes a temporary file and downloads it
function download(filename: string, text: string) {
    var element = document.createElement('a');
    element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    element.setAttribute('download', filename);

    element.style.display = 'none';
    document.body.appendChild(element);

    element.click();

    document.body.removeChild(element);
}


export default Runner;