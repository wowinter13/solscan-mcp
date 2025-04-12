use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenTransferRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_amount_zero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenDefiActivitiesRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenMetaMultiRequest {
    pub address: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenPriceMultiRequest {
    pub address: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenTopRequest {
    // No parameters required
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenTrendingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
