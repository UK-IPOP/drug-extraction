use strsim::{damerau_levenshtein, jaro_winkler, levenshtein, osa_distance, sorensen_dice};

struct Input {
    word: String,
    group: Option<String>,
}

impl Input {
    fn search_word(&self) -> &str {
        &self.word
    }
}

pub enum MetricType {
    Metric,
    Similarity,
}

pub struct DistanceResult {
    value: f64,
}

pub struct Metric {
    name: String,
}

impl Metric {
    // distance is quicker but will only give you
    // edits or similarity
    fn distance(&self, source: &str, target: &str) -> DistanceResult {
        let value = damerau_levenshtein(source, target) as f64;
        DistanceResult { value }
    }
    // search handles more than distance
    // search will give you a FULL result
}

fn initialize_metric() -> Metric {
    let m = Metric {
        name: "damerau".to_string(),
    };
    m
}

#[cfg(test)]
mod tests {
    use crate::models::initialize_metric;

    #[test]
    fn test_initialize_metric() {
        initialize_metric();
        assert_eq!(true, true)
    }
}



