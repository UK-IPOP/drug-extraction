import { Card, FormElement, Input } from "@nextui-org/react";
import * as React from "react";
import { Text } from "@nextui-org/react";

interface LimiterProps {
    edits: boolean,
    onSelected: (newValue: React.ChangeEvent<FormElement>) => void
};


const Limiter = ({ edits, onSelected }: LimiterProps): JSX.Element => {
    if (edits) {
        return (
            <Card>
                <Text small em>Setting to 0 will return only exact matches</Text>
                <Input type="number" fullWidth size="lg" underlined color="primary" label="Maximum edits:" labelLeft="Edits:" helperText="Select a value between 0-5" onChange={onSelected} initialValue={"1"} />
            </Card>
        )
    } else {
        return (
            <Card>
                <Text small em>Setting to 1.0 will return only exact matches</Text>
                <Input underlined fullWidth size="lg" color="primary" label="Minimum similarity:" labelLeft="Similarity:" type="number" helperText="Select a value between 0.0-1.0" onChange={onSelected} initialValue={"0.9"} />
            </Card>
        )
    }

}

export default Limiter;