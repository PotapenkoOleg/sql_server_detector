use std::collections::HashSet;

// pub fn get_all_math_func() -> HashSet<String> {
//     HashSet::from([
//         "ABS".to_string(),
//         "ACOS".to_string(),
//         "ASIN".to_string(),
//         "ATAN".to_string(),
//         "ATN2".to_string(),
//         "CEILING".to_string(),
//         "COS".to_string(),
//         "COT".to_string(),
//         "DEGREES".to_string(),
//         "EXP".to_string(),
//         "FLOOR".to_string(),
//         "LOG".to_string(),
//         "LOG10".to_string(),
//         "PI".to_string(),
//         "POWER".to_string(),
//         "RADIANS".to_string(),
//         "RAND".to_string(),
//         "ROUND".to_string(),
//         "SIGN".to_string(),
//         "SIN".to_string(),
//         "SQRT".to_string(),
//         "SQUARE".to_string(),
//         "TAN".to_string(),
//     ])
// }

pub fn get_sql_server_math_func() -> HashSet<String> {
    HashSet::from([
        "ATN2".to_string(),
        // "SQUARE".to_string() // *
    ])
}
