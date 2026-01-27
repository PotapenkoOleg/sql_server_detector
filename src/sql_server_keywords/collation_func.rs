use std::collections::HashSet;

pub fn get_all_collation_func() -> HashSet<String> {
    HashSet::from([
        "COLLATIONPROPERTY".to_string(),
        "TERTIARY_WEIGHTS".to_string(),
    ])
}

pub fn get_sql_collation_func() -> HashSet<String> {
    HashSet::from([
        "COLLATIONPROPERTY".to_string(),
        "TERTIARY_WEIGHTS".to_string(),
    ])
}
