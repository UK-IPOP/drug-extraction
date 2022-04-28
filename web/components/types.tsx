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
    filterType: string;
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
    searchType: string;
    maxEdits?: number;
    minThresh?: number;
    searchWords?: string[];
    drugList?: Drug[];
}

interface AlgorithmOutputSimple {
    recordId: string,
    algorithm: string,
    edits: number,
    similarity: number,
    searchTerm: string,
    matchedTerm: string,
}

interface AlgorithmOutputDrug {
    recordId: string,
    algorithm: string,
    edits?: number,
    similarity?: number,
    drugName: string,
    drugRxID: string,
    drugClassID: string,
    matchedTerm: string,
}

export type {
    Drug,
    Phase2Options,
    Phase3Options,
    AlgorithmInput,
    AlgorithmOutputSimple,
    AlgorithmOutputDrug,
}