use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenMetaRequest {
    pub token_address: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenMarketsRequest {
    pub token_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenHoldersRequest {
    pub token_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenPriceRequest {
    pub token_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TokenAccountsRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct DefiActivitiesRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_tx: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct BalanceChangeRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_account: Option<String>,
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
    pub remove_spam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_tx: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountDetailRequest {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountTransferRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_amount_zero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
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
pub struct AccountDefiActivitiesRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
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
pub struct AccountTransactionsRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountPortfolioRequest {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountTokenAccountsRequest {
    pub address: String,
    pub r#type: String, // Using r# prefix to use 'type' as a field name since it's a keyword in Rust
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_zero: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountStakeRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountRewardExportRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountTransferExportRequest {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_amount_zero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AccountMetadataRequest {
    pub address: String,
}
