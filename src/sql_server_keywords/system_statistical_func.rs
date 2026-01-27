// System Statistical
// @@CONNECTIONS
// @@CPU_BUSY
// @@IDLE
// @@IO_BUSY
// @@PACK_SENT
// @@PACKET_ERRORS
// @@TIMETICKS
// @@TOTAL_ERRORS
// @@TOTAL_READ
// @@TOTAL_WRITE

use std::collections::HashSet;

pub fn get_all_system_statistical_func() -> HashSet<String> {
    HashSet::from([
        //"COLLATIONPROPERTY".to_string(),

    ])
}

pub fn get_sql_server_system_statistical_func() -> HashSet<String> {
    HashSet::from([
        //"COLLATIONPROPERTY".to_string(),

    ])
}