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


/// SREM -- remove matching member(s) from set
pub fn srem<K: Into<String>, M: IntoIterator<Item=String>>(key: K, members: M) -> RespValue {
    let command = String::from("SREM");
    let values: Vec<RespValue> = Some(command).into_iter().chain(Some(key.into()))
        .chain(members).map(|elem| elem.into()).collect();
    RespValue::Array(values)
}


/// RPOP -- right-handed pop from a member from list
pub fn rpop<K: Into<String>>(key: K) -> RespValue {
    let key: String = key.into();
    resp_array!["RPOP",key]
}


/// LPUSH -- left-handed push member(s) to list
pub fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(key: K, values: V) -> RespValue {
    let command = String::from("LPUSH");
    let values: Vec<RespValue> = Some(command).into_iter().chain(Some(key.into()))
        .chain(values).map(|elem| elem.into()).collect();
    RespValue::Array(values)
}

