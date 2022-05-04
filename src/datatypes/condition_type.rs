use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionType {
    Limit,
    Market,
    Stop,
    StopLimit,
    Trail,
}
