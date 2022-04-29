use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Health {
    Normal,
    Busy,
    VeryBusy,
    SuperBusy,
    NoOrder,
    Stop
}