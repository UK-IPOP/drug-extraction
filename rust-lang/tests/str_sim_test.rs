#[test]
fn test_levenshtein() {
    let s1 = "alcohol";
    let s2 = "acloholism";
    let d = strsim::levenshtein(s1, s2);
    assert_eq!(d, 5);
}

#[test]
fn test_jaro_winkler() {
    let s1 = "alcohol";
    let s2 = "acloholism";
    let d = strsim::jaro_winkler(s1, s2);
    assert_eq!(d, 0.867142857142857);
}

#[test]
fn test_sorensen_dice() {
    let s1 = "alcohol";
    let s2 = "acloholism";
    let d = strsim::sorensen_dice(s1, s2);
    assert_eq!(d, 0.4);
}

#[test]
fn test_damerau() {
    let s1 = "alcohol";
    let s2 = "acloholism";
    let d = strsim::damerau_levenshtein(s1, s2);
    assert_eq!(d, 4);
}

#[test]
fn test_osa() {
    let s1 = "alcohol";
    let s2 = "acloholism";
    let d = strsim::osa_distance(s1, s2);
    assert_eq!(d, 4);
}
