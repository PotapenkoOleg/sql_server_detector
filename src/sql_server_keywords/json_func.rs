use std::collections::HashSet;

// pub fn get_all_json_func() -> HashSet<String> {
//     HashSet::from([
//         "JSON".to_string(),
//         "ISJSON".to_string(),
//         "JSON_ARRAY".to_string(),
//         "JSON_ARRAYAGG".to_string(),
//         "JSON_CONTAINS".to_string(),
//         "JSON_MODIFY".to_string(),
//         "JSON_OBJECT".to_string(),
//         "JSON_OBJECTAGG".to_string(),
//         "JSON_PATH_EXISTS".to_string(),
//         "JSON_QUERY".to_string(),
//         "JSON_VALUE".to_string(),
//     ])
// }

pub fn get_sql_server_json_func() -> HashSet<String> {
    HashSet::from([
        "ISJSON".to_string(),
        "JSON_MODIFY".to_string(),
        "JSON_QUERY".to_string(),
        "JSON_VALUE".to_string(),
    ])
}
