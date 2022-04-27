import { Button, FormElement, Textarea } from "@nextui-org/react";
import * as React from "react";

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
        <div>
            <Textarea disabled={disabled} label={label} placeholder="covid|alcohol|cocaine" initialValue="covid|alcohol|cocaine" onChange={handleText} />
            {!submitted && <Button onClick={handleSubmit}>Accept Search Words</Button>}
        </div>
    )
}


export default SimpleInput;
