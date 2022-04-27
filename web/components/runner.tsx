import { Button, Loading } from "@nextui-org/react";
import Link from "next/link";
import * as React from "react";
import { AlgorithmInput, AlgorithmOutputDense } from "./types";
import Execute from "./main_program";

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

interface RunnerProps {
    inputData: AlgorithmInput;
};

const run_program = async (inputData: AlgorithmInput) => {
    // this is where we could insert wasm code
    // but for now we'll just sleep for a bit
    await sleep(5000);
    return;
};

const Runner = ({ inputData }: RunnerProps): JSX.Element => {
    const [completed, setCompleted] = React.useState<boolean>(false);

    const results = Execute(inputData)

    postFile(results).then(json => {
        console.log(json);
        setCompleted(true);
    });


    if (completed) {
        return (
            <div>
                <h1>Done!</h1>
                <Button color="primary" size="lg"
                    type="submit"
                    flat
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
                                window.location.href = "/completed";
                            });
                    }}
                > Download results
                </Button>
                <Link href="/">Back to home</Link>
            </div >
        )
    }
    return (
        <div>
            <h1>Runner</h1>
            <Loading type="points" />
        </div >
    )
}

const postFile = async (data: AlgorithmOutputDense[]) => {
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