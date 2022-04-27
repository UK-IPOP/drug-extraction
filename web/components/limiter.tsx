import * as React from "react";
import Select from 'react-select'


interface LimiterProps {
    algorithm: string,
    onSelected: (newValue: { value: number, label: number, kind: string }) => void
};


const Limiter = ({ algorithm, onSelected }: LimiterProps): JSX.Element => {
    if (algorithm == "Levenshtein" || algorithm == "Damerau" || algorithm == "OSA") {
        const options = [
            { value: 0, label: 0, kind: "distance" },
            { value: 1, label: 1, kind: "distance" },
            { value: 2, label: 2, kind: "distance" },
            { value: 3, label: 3, kind: "distance" },
            { value: 4, label: 4, kind: "distance" },
            { value: 5, label: 5, kind: "distance" },
        ];
        return (
            <div>
                <label>Maximum edits:</label>
                <Select options={options} isSearchable={true} placeholder="Distance" onChange={onSelected} />
            </div>
        )
    } else if (algorithm == "JaroWinkler" || algorithm == "SorensenDice") {
        const options = [
            { value: 0.5, label: 0.5, kind: "similarity" },
            { value: 0.6, label: 0.6, kind: "similarity" },
            { value: 0.7, label: 0.7, kind: "similarity" },
            { value: 0.8, label: 0.8, kind: "similarity" },
            { value: 0.9, label: 0.9, kind: "similarity" },
            { value: 1.0, label: 1.0, kind: "similarity" },
        ];
        return (
            <div>
                <label>Minimum similarity:</label>
                <Select options={options} isSearchable={true} placeholder="Threshold" onChange={onSelected} />
            </div>
        )
    }

}

export default Limiter;