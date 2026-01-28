use std::collections::HashSet;

// pub fn get_all_data_type_func() -> HashSet<String> {
//     HashSet::from([
//         "DATALENGTH".to_string(),
//         "IDENT_CURRENT".to_string(),
//         "IDENT_INCR".to_string(),
//         "IDENT_SEED".to_string(),
//         "IDENTITY".to_string(),
//         "SQL_VARIANT_PROPERTY".to_string(),
//     ])
// }

pub fn get_sql_server_data_type_func() -> HashSet<String> {
    HashSet::from([
        "DATALENGTH".to_string(),
        "IDENT_CURRENT".to_string(),
        "IDENT_INCR".to_string(),
        "IDENT_SEED".to_string(),
        "IDENTITY".to_string(),
        "SQL_VARIANT_PROPERTY".to_string(),
    ])
}
