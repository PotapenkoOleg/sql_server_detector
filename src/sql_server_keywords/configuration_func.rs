use std::collections::HashSet;

// pub fn get_all_configuration_func() -> HashSet<String> {
//     HashSet::from([
//         "@@DBTS".to_string(),
//         "@@LANGID".to_string(),
//         "@@LANGUAGE".to_string(),
//         "@@LOCK_TIMEOUT".to_string(),
//         "@@MAX_CONNECTIONS".to_string(),
//         "@@MAX_PRECISION".to_string(),
//         "@@NESTLEVEL".to_string(),
//         "@@OPTIONS".to_string(),
//         "@@REMSERVER".to_string(),
//         "@@SERVERNAME".to_string(),
//         "@@SERVICENAME".to_string(),
//         "@@SPID".to_string(),
//         "@@TEXTSIZE".to_string(),
//         "@@VERSION".to_string(),
//     ])
// }

pub fn get_sql_configuration_func() -> HashSet<String> {
    HashSet::from([
        "@@DBTS".to_string(),
        "@@LANGID".to_string(),
        "@@LANGUAGE".to_string(),
        "@@LOCK_TIMEOUT".to_string(),
        "@@MAX_CONNECTIONS".to_string(),
        "@@MAX_PRECISION".to_string(),
        "@@NESTLEVEL".to_string(),
        "@@OPTIONS".to_string(),
        "@@REMSERVER".to_string(),
        "@@SERVERNAME".to_string(),
        "@@SERVICENAME".to_string(),
        "@@SPID".to_string(),
        "@@TEXTSIZE".to_string(),
        "@@VERSION".to_string(),
    ])
}
