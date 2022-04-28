import { Spacer } from "@nextui-org/react";
import React from "react";

type FileProps = {
    onFileSubmit: (event: React.ChangeEvent<HTMLInputElement>) => void;
}

const DataFileUpload = ({ onFileSubmit }: FileProps): JSX.Element => {

    return (
        <div>
            <br />
            <input
                type="file"
                accept=".csv"
                onChange={onFileSubmit}
            />
        </div>)
}

export default DataFileUpload;