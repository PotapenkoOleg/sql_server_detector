use std::collections::HashSet;

pub fn get_all_analytic_func() -> HashSet<String> {
    HashSet::from([
        "CUME_DIST".to_string(),
        "FIRST_VALUE".to_string(),
        "LAG".to_string(),
        "LAST_VALUE".to_string(),
        "LEAD".to_string(),
        "PERCENTILE_CONT".to_string(),
        "PERCENTILE_DISC".to_string(),
        "PERCENT_RANK".to_string(),
    ])
}

pub fn get_sql_server_analytic_func() -> HashSet<String> {
    HashSet::new()
}
