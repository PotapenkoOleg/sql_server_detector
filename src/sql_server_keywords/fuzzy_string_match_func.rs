use std::collections::HashSet;

// pub fn get_all_fuzzy_string_match_func() -> HashSet<String> {
//     HashSet::from([
//         "EDIT_DISTANCE".to_string(),
//         "EDIT_DISTANCE_SIMILARITY".to_string(),
//         "JARO_WINKLER_DISTANCE".to_string(),
//         "JARO_WINKLER_SIMILARITY".to_string(),
//     ])
// }

pub fn get_sql_server_fuzzy_string_match_func() -> HashSet<String> {
    HashSet::from([
        "EDIT_DISTANCE".to_string(),
        "EDIT_DISTANCE_SIMILARITY".to_string(),
        "JARO_WINKLER_DISTANCE".to_string(),
        "JARO_WINKLER_SIMILARITY".to_string(),
    ])
}
