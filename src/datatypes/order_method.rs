use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum OrderMethod {
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "IFD")]
    IfDone,
    #[serde(rename = "OCO")]
    OneCancelsTheOther,
    #[serde(rename = "IFDOCO")]
    IfDoneOneCancelsTheOther,
}
