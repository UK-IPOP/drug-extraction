import { number } from "prop-types";
import { AlgorithmInput, AlgorithmOutputDrug, AlgorithmOutputSimple, Drug } from "./types";

const levenshtein = require("fast-levenshtein");

const SearchSimple = (text: string, targets: string[], id: string, maxEdits?: number, threshold?: number): AlgorithmOutputSimple[] => {
    const results: AlgorithmOutputSimple[] = [];
    targets.forEach((target) => {
        text.split(" ").forEach((word) => {
            const cleanWord = word.toLocaleUpperCase().trim();
            const cleanTarget = target.toLocaleUpperCase().trim();
            const distance = levenshtein.get(cleanWord, cleanTarget);
            const similarity = 1 - (distance / (Math.max(cleanWord.length, cleanTarget.length)));
            if (maxEdits && distance <= maxEdits) {
                const result = {
                    recordId: id,
                    algorithm: "LEVENSHTEIN",
                    edits: distance,
                    similarity: similarity,
                    searchTerm: cleanTarget,
                    matchedTerm: cleanWord,
                };
                results.push(result);
            } else if (threshold && similarity >= threshold) {
                const result = {
                    recordId: id,
                    algorithm: "LEVENSHTEIN",
                    edits: distance,
                    similarity: similarity,
                    searchTerm: cleanTarget,
                    matchedTerm: cleanWord,
                };
                results.push(result);
            }
        });
    });
    return results;
}

const SearchDrug = (text: string, targets: Drug[], id: string, maxEdits?: number, threshold?: number): AlgorithmOutputDrug[] => {
    const results: AlgorithmOutputDrug[] = [];
    targets.forEach((target) => {
        const drugNames = target.name.split("/").map((name) => name.trim().toLocaleUpperCase());
        drugNames.forEach((drugName) => {
            text.split(" ").forEach((word) => {
                const cleanWord = word.toLocaleUpperCase().trim();
                const distance = levenshtein.get(cleanWord, drugName);
                const similarity = 1 - (distance / (Math.max(cleanWord.length, drugName.length)));
                if (maxEdits && distance <= maxEdits) {
                    const result = {
                        recordId: id,
                        algorithm: "LEVENSHTEIN",
                        edits: distance,
                        drugName: drugName,
                        drugRxID: target.rxID,
                        drugClassID: target.classID,
                        matchedTerm: cleanWord,
                    };
                    results.push(result);
                } else if (threshold && similarity >= threshold) {
                    const result = {
                        recordId: id,
                        algorithm: "LEVENSHTEIN",
                        edits: distance,
                        drugName: drugName,
                        drugRxID: target.rxID,
                        drugClassID: target.classID,
                        matchedTerm: cleanWord,
                    };
                    results.push(result);
                }
            });
        })
    });
    return results;
}


const Execute = (inputData: AlgorithmInput): AlgorithmOutputSimple[] | AlgorithmOutputDrug[] => {
    if (inputData.drugList && inputData.drugList.length > 0) {
        const results: AlgorithmOutputDrug[] = [];
        const drugs = inputData.drugList;
        inputData.data.map((row) => {
            const text = row[inputData.targetColumnIndex];
            const id = row[inputData.idColumnIndex];
            const drugResults = SearchDrug(text, drugs, id, inputData.maxEdits, undefined);
            results.push(...drugResults);
        });
        return results;
    } else {
        const results: AlgorithmOutputSimple[] = [];
        inputData.data.map((row) => {
            const text = row[inputData.targetColumnIndex];
            const id = row[inputData.idColumnIndex];
            const targets = inputData.searchWords ? inputData.searchWords : [];
            const simpleResults = SearchSimple(text, targets, id, inputData.maxEdits, inputData.minThresh);
            results.push(...simpleResults);
        });
        return results;
    }
}

export default Execute;