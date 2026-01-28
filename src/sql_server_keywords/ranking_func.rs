use std::collections::HashSet;

// pub fn get_all_ranking_func() -> HashSet<String> {
//     HashSet::from([
//         "DENSE_RANK".to_string(),
//         "NTILE".to_string(),
//         "RANK".to_string(),
//         "ROW_NUMBER".to_string(),
//     ])
// }

pub fn get_sql_server_ranking_func() -> HashSet<String> {
    HashSet::new()
}
