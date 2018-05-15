use common::Role;


/// construct source and destination keys for `conn` lease
/// acquisition attempt
pub fn conn_lease_keys(role: Role) -> (String,String) {
    let mut src = role.to_string();
    src.push_str("::conn-lease");
    let mut dst = role.to_string();
    dst.push_str("::conn-taken");
    (src,dst)
}


/// construct source and destination keys for `auth` lease
/// acquisition attempt
pub fn auth_lease_keys(role: Role) -> (String,String) {
    let mut src = role.to_string();
    src.push_str("::auth-lease");
    let mut dst = role.to_string();
    dst.push_str("::auth-taken");
    (src,dst)
}

