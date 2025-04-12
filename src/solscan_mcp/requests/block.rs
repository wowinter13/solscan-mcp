use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct BlockLastRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct BlockTransactionsRequest {
    pub block: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_vote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct BlockDetailRequest {
    pub block: i64,
}
