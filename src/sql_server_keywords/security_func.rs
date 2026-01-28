use std::collections::HashSet;

// pub fn get_all_security_func() -> HashSet<String> {
//     HashSet::from([
//         "CERTENCODED".to_string(),
//         "CERTPRIVATEKEY".to_string(),
//         "CURRENT_USER".to_string(),
//         "DATABASE_PRINCIPAL_ID".to_string(),
//         "HAS_DBACCESS".to_string(),
//         "HAS_PERMS_BY_NAME".to_string(),
//         "IS_MEMBER".to_string(),
//         "IS_ROLEMEMBER".to_string(),
//         "IS_SRVROLEMEMBER".to_string(),
//         "LOGINPROPERTY".to_string(),
//         "ORIGINAL_LOGIN".to_string(),
//         "PERMISSIONS".to_string(),
//         "PWDENCRYPT".to_string(),
//         "PWDCOMPARE".to_string(),
//         "SESSION_USER".to_string(),
//         "SESSIONPROPERTY".to_string(),
//         "SUSER_ID".to_string(),
//         "SUSER_NAME".to_string(),
//         "SUSER_SID".to_string(),
//         "SUSER_SNAME".to_string(),
//         "SYSTEM_USER".to_string(),
//         "USER".to_string(),
//         "USER_ID".to_string(),
//         "USER_NAME".to_string(),
//     ])
// }

pub fn get_sql_server_security_func() -> HashSet<String> {
    HashSet::from([
        "CERTENCODED".to_string(),
        "CERTPRIVATEKEY".to_string(),
        "DATABASE_PRINCIPAL_ID".to_string(),
        "HAS_DBACCESS".to_string(),
        "HAS_PERMS_BY_NAME".to_string(),
        "IS_MEMBER".to_string(),
        "IS_ROLEMEMBER".to_string(),
        "IS_SRVROLEMEMBER".to_string(),
        "LOGINPROPERTY".to_string(),
        "ORIGINAL_LOGIN".to_string(),
        "PERMISSIONS".to_string(),
        "PWDENCRYPT".to_string(),
        "PWDCOMPARE".to_string(),
        "SESSIONPROPERTY".to_string(),
        "SUSER_ID".to_string(),
        "SUSER_NAME".to_string(),
        "SUSER_SID".to_string(),
        "SUSER_SNAME".to_string(),
        "USER_ID".to_string(),
        "USER_NAME".to_string(),
    ])
}
