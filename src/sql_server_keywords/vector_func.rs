use std::collections::HashSet;

// pub fn get_all_vector_func() -> HashSet<String> {
//     HashSet::from([
//         "VECTOR_DISTANCE".to_string(),
//         "VECTOR_NORM".to_string(),
//         "VECTOR_NORMALIZE".to_string(),
//         "VECTORPROPERTY".to_string(),
//         "VECTOR_SEARCH".to_string(),
//     ])
// }

pub fn get_sql_server_vector_func() -> HashSet<String> {
    HashSet::from([
        "VECTOR_DISTANCE".to_string(),
        "VECTOR_NORM".to_string(),
        "VECTOR_NORMALIZE".to_string(),
        "VECTORPROPERTY".to_string(),
        "VECTOR_SEARCH".to_string(),
    ])
}
