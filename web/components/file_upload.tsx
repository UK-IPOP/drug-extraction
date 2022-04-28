import { Grid, Spacer, Text } from "@nextui-org/react";
import * as React from "react";
import Button from '@mui/material/Button';
import CloudUpload from '@mui/icons-material/CloudUpload';

type FileProps = {
    onFileSubmit: (event: React.ChangeEvent<HTMLInputElement>) => void;
}

const DataFileUpload = ({ onFileSubmit }: FileProps): JSX.Element => {

    return (
        <Grid>
            <Grid>
                <Button
                    variant="outlined"
                    component="label"
                    endIcon={<CloudUpload />}
                    size="large"
                >
                    Upload File
                    <input
                        type="file"
                        accept=".csv"
                        hidden
                        onChange={onFileSubmit}
                    />
                </Button>
            </Grid>
            <Grid>

            </Grid>
        </Grid >)
}



export default DataFileUpload;