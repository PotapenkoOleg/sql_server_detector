use std::collections::HashSet;

// pub fn get_all_trigger_func() -> HashSet<String> {
//     HashSet::from([
//         "COLUMNS_UPDATED".to_string(),
//         "EVENTDATA".to_string(),
//         "TRIGGER_NESTLEVEL".to_string(),
//         "UPDATE".to_string(),
//     ])
// }

pub fn get_sql_server_trigger_func() -> HashSet<String> {
    HashSet::from([
        "COLUMNS_UPDATED".to_string(),
        "EVENTDATA".to_string(),
        "TRIGGER_NESTLEVEL".to_string(),
        // "UPDATE".to_string(), // *
    ])
}
