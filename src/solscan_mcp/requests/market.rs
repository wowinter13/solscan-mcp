use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MarketListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MarketInfoRequest {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MarketVolumeRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Vec<String>>,
}
