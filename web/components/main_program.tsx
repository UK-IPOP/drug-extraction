import { AlgorithmInput, AlgorithmOutputDense } from "./types";

const levenshtein = require("fast-levenshtein");

const SearchSimple = (text: string, targets: string[], maxEdits: number, id: string): AlgorithmOutputDense[] => {
    const results: AlgorithmOutputDense[] = [];
    targets.forEach((target) => {
        text.split(" ").forEach((word) => {
            const cleanWord = word.toLocaleUpperCase().trim();
            const cleanTarget = target.toLocaleUpperCase().trim();
            const distance = levenshtein.get(cleanWord, cleanTarget);
            if (distance <= maxEdits) {
                const result = {
                    recordId: id,
                    algorithm: "LEVENSHTEIN",
                    edits: distance,
                    searchTerm: cleanTarget,
                    matchedTerm: cleanWord,
                };
                results.push(result);
            }
        });
    });
    return results;
}


const Execute = (inputData: AlgorithmInput): AlgorithmOutputDense[] => {
    // iterate rows
    const results: AlgorithmOutputDense[] = [];
    inputData.data.map((row) => {
        const text = row[inputData.targetColumnIndex];
        const id = row[inputData.idColumnIndex];
        const targets = inputData.searchWords;
        const searchResults = SearchSimple(text, targets, inputData.maxEdits, id);
        results.push(...searchResults);
    });
    console.log(results[0]);
    return results;
}

export default Execute;