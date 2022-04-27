import * as React from "react";
import Select from 'react-select'


// https://react-select.com/props

interface SelectorProps {
    optionsList: string[],
    placeholder: string,
    clearable?: boolean,
    onSelected: (newValue: { value: number, label: string }) => void
};


const Selector = ({ optionsList, placeholder, clearable = true, onSelected }: SelectorProps): JSX.Element => {
    const prepared = optionsList.map((option: string, index: number) => { return { value: index, label: option } });
    return <Select options={prepared} isSearchable={true} isClearable={clearable} placeholder={placeholder} onChange={onSelected} />
}

export default Selector;