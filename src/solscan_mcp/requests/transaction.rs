use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TransactionDetailRequest {
    pub tx: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TransactionActionsRequest {
    pub tx: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TransactionLastRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
