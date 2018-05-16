use serde_json::Value;

simple_unit!(
    RpcMethod, "supported rpc methods",
    EthCall => "eth_call",
    SendRawTx => "eth_sendRawTransaction",
    EstimateGas => "eth_estimateGas",
    BlockNumber => "eth_blockNumber",
    BlockByNumber => "eth_getBlockByNumber",
    BlockByHash => "eth_getBlockByHash",
    TxByHash => "eth_getTransactionByHash",
    TxByBlockHashAndIndex => "eth_getTransactionByBlockHashAndIndex",
    TxByBlockNumberAndIndex => "eth_getTransactionByBlockNumberAndIndex",
    StorageAt => "eth_getStorageAt",
    GasPrice => "eth_gasPrice",
    TxCount => "eth_getTransactionCount",
    GetCode => "eth_getCode",
    GetLogs => "eth_getLogs",
    Balance => "eth_getBalance",
    Accounts => "eth_accounts",
    Syncing => "eth_syncing",
    NetVersion => "net_version",
);


impl RpcMethod {

    /// get index of default block parameter if
    /// one exists.
    ///
    /// this funciton is used by oracle implementations to
    /// override calls to the "latest" block (i.e. those which
    /// explicitly request "latest", or which leave the default block
    /// parameter empty).
    ///
    pub fn default_block_index(&self) -> Option<usize> {
        match *self {
            RpcMethod::EthCall => Some(1),
            RpcMethod::StorageAt => Some(2),
            RpcMethod::GetCode => Some(1),
            RpcMethod::TxCount => Some(1),
            RpcMethod::Balance => Some(1),
            _ => None,
        }
    }

    /// get static result if any
    ///
    /// if `Some` variant is returned, skip querying the node
    /// and use the supplied value as the `result` of the
    /// rpc call.  used to short-circuit calls which have fixed
    /// return values (e.g. `eth_accounts` will always be an empty
    /// array).
    ///
    // NOTE: short-circuiting the result of non-applicable rpc calls
    // is not a valid substitute for locking down the actual node
    // instance.  live in fear!
    pub fn static_result(&self) -> Option<Value> {
        match *self {
            RpcMethod::Accounts => Some(Value::Array(Vec::new())),
            RpcMethod::Syncing => Some(Value::Bool(false)),
            _ => None,
        }
    }
}

