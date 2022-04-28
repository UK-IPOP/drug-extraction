import { Button, Col, Container, Grid, Row, Spacer } from "@nextui-org/react";
import * as React from "react";
import DataFileUpload from "../file_upload";
import styles from '../../styles/Home.module.css';
import { Text } from "@nextui-org/react";
import ArrowForwardIcon from '@mui/icons-material/ArrowForward';

interface Phase1Props {
    dataHandler: (data: string[][], headerRow: string[]) => void

};

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const Phase1Component = ({ dataHandler }: Phase1Props): JSX.Element => {
    const [fileName, setFileName] = React.useState<string>("");
    const [data, _] = React.useState<string[][]>([]);
    const [headerRow, setHeaderRow] = React.useState<string[]>([]);
    const [fileUploaded, setFileUploaded] = React.useState<boolean>(false);

    const handleFile = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files) {
            const file = e.target.files[0];
            setFileName(file.name);
            const reader = new FileReader();
            reader.onload = () => {
                const text = reader.result as string;
                text.split('\n').map((line, i) => {
                    if (i == 0) {
                        setHeaderRow(line.split(','));
                    } else {
                        const row = line.split(',');
                        data.push(row);
                    }
                });
            };
            reader.readAsText(file);
            setFileUploaded(true);
        }
    }

    if (fileUploaded) {
        return (
            <Grid.Container justify="center">
                <Grid xs={12} justify="center">
                    <Text small em>Uploaded: {fileName}</Text>
                </Grid>
                <Grid xs={12} justify="center">
                    <Button rounded onClick={() => dataHandler(data, headerRow)}><ArrowForwardIcon /></Button>
                </Grid>
            </Grid.Container>
        )
    }
    return (
        <Grid.Container justify="center">
            <Grid xs={12} justify="center">
                <Text h2 className={styles.subtitle}>Get started by uploading a file:</Text>
            </Grid>
            <Grid xs={12} justify="center">
                <DataFileUpload onFileSubmit={handleFile} />
            </Grid>
            <Spacer x={2} />
        </Grid.Container >
    );

}


export default Phase1Component;
