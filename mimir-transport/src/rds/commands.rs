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

#[cfg(test)]
mod test {
    use super::*;

    fn vec_of_strings(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| (*s).into()).collect()
    }

    fn bulk_string<Str: AsRef<str>>(string: Str) -> RespValue {
        RespValue::BulkString(string.as_ref().as_bytes().to_vec())
    }

    #[test]
    fn test_brpop() {

        let keys = ["k1", "k2"];

        let resp = brpop(vec_of_strings(&keys), Some(5));

        let expected_resp = RespValue::Array(vec![
            bulk_string("BRPOP"),
            bulk_string(&keys[0]),
            bulk_string(&keys[1]),
            bulk_string("5")
        ]);
        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_sadd() {

        let key = "k1";
        let members = ["m1", "m2"];

        let resp = sadd(key, vec_of_strings(&members));

        let expected_resp = RespValue::Array(vec![
            bulk_string("SADD"),
            bulk_string(&key),
            bulk_string(&members[0]),
            bulk_string(&members[1])
        ]);
        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_srem() {

        let key = "k1";
        let members = ["m1", "m2"];

        let resp = srem(key, vec_of_strings(&members));

        let expected_resp = RespValue::Array(vec![
            bulk_string("SREM"),
            bulk_string(&key),
            bulk_string(&members[0]),
            bulk_string(&members[1])
        ]);
        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_smove() {

        let key_src = "k1";
        let key_dst = "k2";
        let member = "m";

        let resp = smove(key_src, key_dst, member);

        let expected_resp = RespValue::Array(vec![
            bulk_string("SMOVE"),
            bulk_string(&key_src),
            bulk_string(&key_dst),
            bulk_string(&member),
        ]);
        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_rpop() {

        let key = "k1";

        let resp = rpop(key.to_owned());

        let expected_resp = RespValue::Array(vec![
            bulk_string("RPOP"),
            bulk_string(key)
        ]);
        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_lpush() {

        let key = "k1";
        let members = ["m1", "m2"];

        let resp = lpush(key, vec_of_strings(&members));

        let expected_resp = RespValue::Array(vec![
            bulk_string("LPUSH"),
            bulk_string(&key),
            bulk_string(&members[0]),
            bulk_string(&members[1])
        ]);
        assert_eq!(resp, expected_resp);
    }
}
