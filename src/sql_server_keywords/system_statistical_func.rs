use std::collections::HashSet;

// pub fn get_all_system_statistical_func() -> HashSet<String> {
//     HashSet::from([
//         "@@CONNECTIONS".to_string(),
//         "@@CPU_BUSY".to_string(),
//         "@@IDLE".to_string(),
//         "@@IO_BUSY".to_string(),
//         "@@PACK_SENT".to_string(),
//         "@@PACKET_ERRORS".to_string(),
//         "@@TIMETICKS".to_string(),
//         "@@TOTAL_ERRORS".to_string(),
//         "@@TOTAL_READ".to_string(),
//         "@@TOTAL_WRITE".to_string(),
//     ])
// }

pub fn get_sql_server_system_statistical_func() -> HashSet<String> {
    HashSet::from([
        "@@CONNECTIONS".to_string(),
        "@@CPU_BUSY".to_string(),
        "@@IDLE".to_string(),
        "@@IO_BUSY".to_string(),
        "@@PACK_SENT".to_string(),
        "@@PACKET_ERRORS".to_string(),
        "@@TIMETICKS".to_string(),
        "@@TOTAL_ERRORS".to_string(),
        "@@TOTAL_READ".to_string(),
        "@@TOTAL_WRITE".to_string(),
    ])
}
