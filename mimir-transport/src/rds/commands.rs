use redis_async::resp::RespValue;


// -------------------------- blocking ops --------------------------


/// BRPOP -- blocking right-handed pop from one or more lists
pub fn brpop<K: IntoIterator<Item=String>>(keys: K, timeout: Option<u64>) -> RespValue {
    let command = String::from("BRPOP");
    let timeout = timeout.unwrap_or(0).to_string();
    let values: Vec<RespValue> = Some(command).into_iter().chain(keys)
        .chain(Some(timeout)).map(|elem| elem.into()).collect();
    RespValue::Array(values)
}


// ------------------------ non-blocking ops ------------------------


/// SADD -- add member(s) to set
pub fn sadd<K: Into<String>, M: IntoIterator<Item=String>>(key: K, members: M) -> RespValue {
    let command = String::from("SADD");
    let values: Vec<RespValue> = Some(command).into_iter().chain(Some(key.into()))
        .chain(members).map(|elem| elem.into()).collect();
    RespValue::Array(values)
}


/// SREM -- remove matching member(s) from set
pub fn srem<K: Into<String>, M: IntoIterator<Item=String>>(key: K, members: M) -> RespValue {
    let command = String::from("SREM");
    let values: Vec<RespValue> = Some(command).into_iter().chain(Some(key.into()))
        .chain(members).map(|elem| elem.into()).collect();
    RespValue::Array(values)
}


/// SMOVE -- move matching member from one set to another
pub fn smove<K: Into<String>, M: Into<String>>(src: K, dst: K, member: M) -> RespValue {
    resp_array!["SMOVE",src.into(),dst.into(),member.into()]
}


/// RPOP -- right-handed pop from a member from list
pub fn rpop<K: Into<String>>(key: K) -> RespValue {
    resp_array!["RPOP",key.into()]
}


/// LPUSH -- left-handed push member(s) to list
pub fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(key: K, values: V) -> RespValue {
    let command = String::from("LPUSH");
    let values: Vec<RespValue> = Some(command).into_iter().chain(Some(key.into()))
        .chain(values).map(|elem| elem.into()).collect();
    RespValue::Array(values)
}
