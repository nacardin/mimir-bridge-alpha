extern crate mimir_crypto;
extern crate mimir_proto;
extern crate serde_json;
extern crate rand;



fn main() {
    // generate a random message w/ no certs.
    let mut message = gen::message();

    // apply some certs to message.
    apply::certs(&mut message,6);

    // get message as json string.
    let msg_json = serde_json::to_string(&message).unwrap();
    
    // give the people what they want.
    println!("{}",msg_json);
}


/// generate random values.
mod gen { 
    use mimir_crypto::secp256k1::Signer;
    use mimir_proto::message::cert::Clear;
    use mimir_proto::message::{
        Payload,
        Message,
    };
    use rand;

    pub fn payload() -> Payload {
        let record = r#"{"method":"eth_gasPrice","result":"0xdeadbeef"}"#.into();
        let (address,number,hash) = rand::random();
        Payload { record, address, number, hash }
    }

    pub fn message() -> Message { Message::new(payload()) }

    pub fn sealer() -> Signer { rand::random() }

    pub fn clear() -> Clear { rand::random() }
}


/// apply random values.
mod apply {
    use mimir_crypto::Keccak256;
    use mimir_proto::seal::Sealer;
    use mimir_proto::message::{
        Message,
        STEP,
    };
    use rand;
    use gen;

    pub fn certs(message: &mut Message, steps: usize) {
        let mut seed = None;
        for index in 0..(steps) {
            match STEP::new(index) {
                STEP::ORACLE => {
                    let sealer = gen::sealer();
                    let cert = sealer.seal_oracle(&message);
                    message.verify.push(cert);
                },
                STEP::NOTARY => {
                    let sealer = gen::sealer();
                    let cert = sealer.seal_notary(&message);
                    message.notary.push(cert);
                },
                STEP::BLIND => {
                    assert!(seed.is_none(),"must consume previous seed first");
                    let clear = gen::clear();
                    let blind = Keccak256::hash(&clear);
                    seed = Some(clear);
                    message.blind.push(blind.into());
                },
                STEP::CLEAR => {
                    assert!(seed.is_some(),"must seed prior to clearing");
                    let clear = seed.take().unwrap();
                    message.blind.push(clear);
                },
                STEP::ROUTE => {
                    let sealer = gen::sealer();
                    let route = rand::random();
                    let cert = sealer.seal_route(&message,route);
                    message.route.push(cert);
                },
                STEP::VERIFY => {
                    let sealer = gen::sealer();
                    let cert = sealer.seal_verify(&message,0x00);
                    message.verify.push(cert);
                },
            }
        }
    }
}
