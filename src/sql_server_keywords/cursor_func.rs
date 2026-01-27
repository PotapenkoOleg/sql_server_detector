use std::collections::HashSet;

pub fn get_all_cursor_func() -> HashSet<String> {
    HashSet::from([
        "@@CURSOR_ROWS".to_string(),
        "@@FETCH_STATUS".to_string(),
        "CURSOR_STATUS".to_string(),
    ])
}

pub fn get_sql_server_cursor_func() -> HashSet<String> {
    HashSet::from([
        "@@CURSOR_ROWS".to_string(),
        "@@FETCH_STATUS".to_string(),
        "CURSOR_STATUS".to_string(),
    ])
}
