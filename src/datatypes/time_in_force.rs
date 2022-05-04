use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GoodTillCancel,
    #[serde(rename = "IOC")]
    ImmediateOrcancel,
    #[serde(rename = "FOK")]
    FillOrKill,
}
