use std::collections::HashSet;

pub fn get_all_agg_func() -> HashSet<String> {
    HashSet::from([
        "APPROX_COUNT_DISTINCT".to_string(),
        "APPROX_PERCENTILE_CONT".to_string(),
        "APPROX_PERCENTILE_DISC".to_string(),
        "AVG".to_string(),
        "CHECKSUM_AGG".to_string(),
        "COUNT".to_string(),
        "COUNT_BIG".to_string(),
        "GROUPING".to_string(),
        "GROUPING_ID".to_string(),
        "MAX".to_string(),
        "MIN".to_string(),
        "PRODUCT".to_string(),
        "STDEV".to_string(),
        "STDEVP".to_string(),
        "SUM".to_string(),
        "VAR".to_string(),
        "VARP".to_string(),
    ])
}

pub fn get_sql_server_agg_func() -> HashSet<String> {
    HashSet::from([
        "APPROX_COUNT_DISTINCT".to_string(),
        "APPROX_PERCENTILE_CONT".to_string(),
        "APPROX_PERCENTILE_DISC".to_string(),
        "CHECKSUM_AGG".to_string(),
        "COUNT_BIG".to_string(),
        //"GROUPING".to_string(), // *
        "GROUPING_ID".to_string(),
        //"PRODUCT".to_string(), // *
        "STDEV".to_string(),
        "STDEVP".to_string(),
        //"VAR".to_string(), // *
        "VARP".to_string(),
    ])
}
