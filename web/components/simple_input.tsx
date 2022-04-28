import { Card, FormElement, Spacer, Textarea } from "@nextui-org/react";
import * as React from "react";
import CheckIcon from '@mui/icons-material/Check';
import Button from '@mui/material/Button'

interface SimpleInputProps {
    submitted: boolean;
    wordHandler: (words: string[],) => void
};


const SimpleInput = ({ submitted, wordHandler }: SimpleInputProps): JSX.Element => {
    const [words, setWords] = React.useState<string[]>([]);
    const [disabled, setDisabled] = React.useState<boolean>(false);
    const [label, setLabel] = React.useState<string>("Simple search:");

    const handleText = (e: React.ChangeEvent<FormElement>) => {
        const newWords = e.target.value.split('|').map(word => word.trim());
        setWords(newWords);
    }

    const handleSubmit = () => {
        setDisabled(true)
        setLabel("Simple search (locked):")
        wordHandler(words);
    }

    return (
        <Card>
            <Textarea
                readOnly={disabled}
                label={label}
                onChange={handleText}
                bordered
                borderWeight="light"
                size="xl"
                helperText="Separate words with a pipe '|' character (i.e. 'alcohol|heroin')."
                minRows={2}
                maxRows={10}
                fullWidth={true}
                color="primary"
            />
            <Spacer y={2} />
            {!submitted &&
                <Button variant="outlined" color="success" onClick={handleSubmit}>Accept Search Words</Button>
            }
        </Card >
    )
}


export default SimpleInput;
