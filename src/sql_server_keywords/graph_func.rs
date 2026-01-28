use std::collections::HashSet;

// pub fn get_all_graph_func() -> HashSet<String> {
//     HashSet::from([
//         "EDGE_ID_FROM_PARTS".to_string(),
//         "GRAPH_ID_FROM_EDGE_ID".to_string(),
//         "GRAPH_ID_FROM_NODE_ID".to_string(),
//         "NODE_ID_FROM_PARTS".to_string(),
//         "OBJECT_ID_FROM_EDGE_ID".to_string(),
//         "OBJECT_ID_FROM_NODE_ID".to_string(),
//     ])
// }

pub fn get_sql_server_graph_func() -> HashSet<String> {
    HashSet::from([
        "EDGE_ID_FROM_PARTS".to_string(),
        "GRAPH_ID_FROM_EDGE_ID".to_string(),
        "GRAPH_ID_FROM_NODE_ID".to_string(),
        "NODE_ID_FROM_PARTS".to_string(),
        "OBJECT_ID_FROM_EDGE_ID".to_string(),
        "OBJECT_ID_FROM_NODE_ID".to_string(),
    ])
}
