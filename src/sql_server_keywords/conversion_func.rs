use std::collections::HashSet;

// pub fn get_all_conversion_func() -> HashSet<String> {
//     HashSet::from([
//         "CAST".to_string(),
//         "CONVERT".to_string(),
//         "PARSE".to_string(),
//         "TRY_CAST".to_string(),
//         "TRY_CONVERT".to_string(),
//         "TRY_PARSE".to_string(),
//     ])
// }

pub fn get_sql_server_conversion_func() -> HashSet<String> {
    HashSet::from([
        "PARSE".to_string(),
        "TRY_CAST".to_string(),
        "TRY_CONVERT".to_string(),
        "TRY_PARSE".to_string(),
    ])
}
