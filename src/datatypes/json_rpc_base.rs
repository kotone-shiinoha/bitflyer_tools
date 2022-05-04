use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcBase<T> {
    jsonrpc: String,
    method: String,
    params: T,
}