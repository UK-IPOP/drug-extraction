import { Button } from "@nextui-org/react";
import * as React from "react";
import DataFileUpload from "../file_upload";

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
        <div>
            <h1>Get started by uploading a file:</h1>
            <DataFileUpload onFileSubmit={handleFile} />
            <Button onClick={() => dataHandler(data, headerRow)}>Continue</Button>
        </div>
    );

}


export default Phase1Component;
