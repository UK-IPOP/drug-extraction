import { Button, Col, Container, Grid, Row, Spacer } from "@nextui-org/react";
import * as React from "react";
import DataFileUpload from "../file_upload";
import styles from '../../styles/Home.module.css';
import { Text } from "@nextui-org/react";

interface Phase1Props {
    dataHandler: (data: string[][], headerRow: string[]) => void

};


const Phase1Component = ({ dataHandler }: Phase1Props): JSX.Element => {
    const [data, _] = React.useState<string[][]>([]);
    const [headerRow, setHeaderRow] = React.useState<string[]>([]);

    const handleFile = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files) {
            const file = e.target.files[0];
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
        }
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
            <Grid xs={12} justify="center">
                <Button rounded onClick={() => dataHandler(data, headerRow)}>Continue</Button>
            </Grid>
        </Grid.Container >
    );

}


export default Phase1Component;
