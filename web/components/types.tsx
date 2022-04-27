interface Drug {
    name: string;
    rxID: string,
    classID: string,
}

interface Phase2Options {
    idColumnIndex: number;
    targetColumnIndex: number;
    algorithm: string;
    searchType: string;
    outputFormat: string;
    analyze: boolean;
}

interface Phase3Options {
    maxEdits?: number;
    minThresh?: number;
    searchWords?: string[];
    drugList?: Drug[];
}

interface AlgorithmInput {
    data: string[][];
    headers: string[];
    idColumnIndex: number;
    targetColumnIndex: number;
    algorithm: string;
    searchType: string;
    outputFormat: string;
    analyze: boolean;
    maxEdits?: number;
    minThresh?: number;
    searchWords?: string[];
    drugList?: Drug[];
}

interface AlgorithmOutputDense {
    recordId: string,
    algorithm: string,
    edits: number,
    searchTerm: string,
    matchedTerm: string,
}

export type {
    Drug,
    Phase2Options,
    Phase3Options,
    AlgorithmInput,
    AlgorithmOutputDense,
}