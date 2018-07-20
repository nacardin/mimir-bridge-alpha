extern crate mimir_transport;
extern crate tokio;

use std::collections::HashMap;
use mimir_transport::service;
use tokio::prelude::*;


#[derive(Debug)]
enum Op<K,V> {
    Put(K,V),
    Take(K),
}


fn main() {

    let work = future::lazy(|| {
        let mut mapping = HashMap::new();
        let handle = service::spawn(move |operation| {
            println!("applying: {:?}",operation);
            let rslt = match operation { 
                Op::Put(key,val) => mapping.insert(key,val),
                Op::Take(key) => mapping.remove(key),
            };
            Ok(rslt)
        });
        let requests = vec![
            handle.call(Op::Put("hello","world")),
            handle.call(Op::Put("hello","world!")),
            handle.call(Op::Take("hello")),
        ];
        future::collect(requests).map(|responses| {
            assert_eq!(responses,vec![None,Some("world"),Some("world!")]);
        }).map_err(|_|())
    });
    tokio::run(work)
}

