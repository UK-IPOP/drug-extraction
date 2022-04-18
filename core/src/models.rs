// use strsim::{damerau_levenshtein, jaro_winkler, levenshtein, osa_distance, sorensen_dice};
//
// struct SimpleExtractor {
//
// };
//
// extractor = E()  // rust code
// results = []
// for line in reader.lines() { // client code
//     result = extractor.extract(line) // rust code
//     results.append(result)
// }
//
// // rust
// data = results.to_json()
//
// // client
// data.to_file()
//
// // extractor initializer
// // params:
// // metric: choice
// // limit: float (similarity) 0-1
// // ! NO merge input file w/ output?
// //  - there are other, better tools for this (excel/pandas)
// // dense or wide-form?
//
// // extractor.extract main component
// // runs the distance function from the specified initializer
// // returns:
// // - search term
// // - matched term
// // - distance/null
// // - similarity score
// // - record id
// // OPTIONALLY if drug-search
// //      - drug group/class
// //        - rx_id
// //          - other identifiers
//
// #[cfg(test)]
// mod tests {
//     use super::*;
// }
