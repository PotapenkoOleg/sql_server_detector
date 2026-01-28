use std::collections::HashSet;

// pub fn get_all_logical_func() -> HashSet<String> {
//     HashSet::from([
//         "CHOOSE".to_string(),
//         "GREATEST".to_string(),
//         "IIF".to_string(),
//         "LEAST".to_string(),
//     ])
// }

pub fn get_sql_server_logical_func() -> HashSet<String> {
    HashSet::from([
        "CHOOSE".to_string(),
        "IIF".to_string(),
    ])
}