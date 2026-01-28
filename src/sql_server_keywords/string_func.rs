use std::collections::HashSet;

// pub fn get_all_string_func() -> HashSet<String> {
//     HashSet::from([
//         "ASCII".to_string(),
//         "CHAR".to_string(),
//         "CHARINDEX".to_string(),
//         "CONCAT".to_string(),
//         "CONCAT_WS".to_string(),
//         "DIFFERENCE".to_string(),
//         "FORMAT".to_string(),
//         "LEFT".to_string(),
//         "LEN".to_string(),
//         "LOWER".to_string(),
//         "LTRIM".to_string(),
//         "NCHAR".to_string(),
//         "PATINDEX".to_string(),
//         "QUOTENAME".to_string(),
//         "REPLACE".to_string(),
//         "REPLICATE".to_string(),
//         "REVERSE".to_string(),
//         "RIGHT".to_string(),
//         "RTRIM".to_string(),
//         "SOUNDEX".to_string(),
//         "SPACE".to_string(),
//         "STR".to_string(),
//         "STRING_AGG".to_string(),
//         "STRING_ESCAPE".to_string(),
//         "STUFF".to_string(),
//         "SUBSTRING".to_string(),
//         "TRANSLATE".to_string(),
//         "TRIM".to_string(),
//         "UNICODE".to_string(),
//         "UNISTR".to_string(),
//         "UPPER".to_string(),
//     ])
// }

pub fn get_sql_server_string_func() -> HashSet<String> {
    HashSet::from([
        "CHARINDEX".to_string(),
        // "DIFFERENCE".to_string(), // *
        // "FORMAT".to_string(), // *
        "PATINDEX".to_string(),
        "QUOTENAME".to_string(),
        "REPLICATE".to_string(),
        "SOUNDEX".to_string(),
        "SPACE".to_string(),
        // "STR".to_string(), // *
        "STRING_ESCAPE".to_string(),
        "STUFF".to_string(),
    ])
}
