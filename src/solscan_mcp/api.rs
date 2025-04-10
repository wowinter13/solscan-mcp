use std::sync::Arc;

use rmcp::{model::*, schemars, tool, Error as McpError, ServerHandler};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::Mutex;

// Base URLs for Solscan API
const SOLSCAN_API_BASE_URL: &str = "https://pro-api.solscan.io/v2.0";
const SOLSCAN_PUBLIC_API_BASE_URL: &str = "https://public-api.solscan.io";
// const WSOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ChainInfoRequest {
    // No parameters
}

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
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_tx: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TransactionDetailRequest {
    pub tx: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TransactionActionsRequest {
    pub tx: String,
}

#[derive(Clone)]
pub struct SolscanApi {
    api_key: Arc<Mutex<String>>,
    client: reqwest::Client,
}

#[tool(tool_box)]
impl SolscanApi {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Arc::new(Mutex::new(api_key)),
            client: reqwest::Client::new(),
        }
    }

    async fn make_request(&self, endpoint: &str, params: Option<Value>) -> Result<Value, McpError> {
        let url = format!("{}{}", SOLSCAN_API_BASE_URL, endpoint);
        let api_key = self.api_key.lock().await.clone();

        let request = self.client.get(&url).header("token", api_key);

        // Add query parameters if provided
        let request = if let Some(params) = params {
            request.query(&params)
        } else {
            request
        };

        let response = request
            .send()
            .await
            .map_err(|e| McpError::internal_error(format!("HTTP request error: {}", e), None))?;

        if response.status() != reqwest::StatusCode::OK {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(McpError::internal_error(
                format!("HTTP {}", error_text),
                None,
            ));
        }

        response
            .json::<Value>()
            .await
            .map_err(|e| McpError::internal_error(format!("JSON parsing error: {}", e), None))
    }

    // Make a request to the Solscan Public API
    async fn make_public_request(&self, endpoint: &str) -> Result<Value, McpError> {
        let url = format!("{}{}", SOLSCAN_PUBLIC_API_BASE_URL, endpoint);
        let api_key = self.api_key.lock().await.clone();

        let request = self.client.get(&url).header("token", api_key);

        let response = request
            .send()
            .await
            .map_err(|e| McpError::internal_error(format!("HTTP request error: {}", e), None))?;

        if response.status() != reqwest::StatusCode::OK {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(McpError::internal_error(
                format!("HTTP {}", error_text),
                None,
            ));
        }

        response
            .json::<Value>()
            .await
            .map_err(|e| McpError::internal_error(format!("JSON parsing error: {}", e), None))
    }

    // Chain Info endpoint
    #[tool(description = "Get Solana blockchain information")]
    async fn chain_info(
        &self,
        #[tool(aggr)] _request: ChainInfoRequest,
    ) -> Result<CallToolResult, McpError> {
        let response = self.make_public_request("/chaininfo").await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Metadata endpoint
    #[tool(description = "Get token metadata")]
    async fn token_meta(
        &self,
        #[tool(aggr)] request: TokenMetaRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "token_address": request.token_address,
        });

        let response = self.make_request("/token/meta", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Markets endpoint
    #[tool(description = "Get token market data and liquidity pools")]
    async fn token_markets(
        &self,
        #[tool(aggr)] request: TokenMarketsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "token_address": request.token_address,
        });

        // Add optional parameters
        if let Some(sort_by) = request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(program) = request.program {
            params["program"] = json!(program);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        let response = self.make_request("/token/markets", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Holders endpoint
    #[tool(description = "Get token holder distribution")]
    async fn token_holders(
        &self,
        #[tool(aggr)] request: TokenHoldersRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "token_address": request.token_address,
        });

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(from_amount) = request.from_amount {
            params["from_amount"] = json!(from_amount);
        }

        if let Some(to_amount) = request.to_amount {
            params["to_amount"] = json!(to_amount);
        }

        let response = self.make_request("/token/holders", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Price endpoint
    #[tool(description = "Get historical token price data")]
    async fn token_price(
        &self,
        #[tool(aggr)] request: TokenPriceRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "token_address": request.token_address,
        });

        // Add optional parameters
        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        let response = self.make_request("/token/price", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // // Token Accounts endpoint
    // #[tool(description = "Get token holdings for a wallet")]
    // async fn token_accounts(
    //     &self,
    //     #[tool(aggr)] request: TokenAccountsRequest,
    // ) -> Result<CallToolResult, McpError> {
    //     let mut params = json!({
    //         "account": request.account,
    //     });

    //     // Add optional parameters
    //     if let Some(page) = request.page {
    //         params["page"] = json!(page);
    //     }

    //     if let Some(page_size) = request.page_size {
    //         params["page_size"] = json!(page_size);
    //     }

    //     let response = self.make_request("/token/accounts", Some(params)).await?;

    //     let content = Content::json(response).map_err(|e| {
    //         McpError::internal_error(
    //             "Failed to serialize JSON response",
    //             Some(json!({"error": e.message})),
    //         )
    //     })?;

    //     Ok(CallToolResult::success(vec![content]))
    // }

    // DeFi Activities endpoint
    // #[tool(description = "Get DeFi activities for a wallet")]
    // async fn defi_activities(
    //     &self,
    //     #[tool(aggr)] request: DefiActivitiesRequest,
    // ) -> Result<CallToolResult, McpError> {
    //     let mut params = json!({
    //         "account": request.account,
    //     });

    //     // Add optional parameters
    //     if let Some(page) = request.page {
    //         params["page"] = json!(page);
    //     }

    //     if let Some(page_size) = request.page_size {
    //         params["page_size"] = json!(page_size);
    //     }

    //     if let Some(before_tx) = request.before_tx {
    //         params["before_tx"] = json!(before_tx);
    //     }

    //     let response = self.make_request("/defi/activities", Some(params)).await?;

    //     let content = Content::json(response).map_err(|e| {
    //         McpError::internal_error(
    //             "Failed to serialize JSON response",
    //             Some(json!({"error": e.message})),
    //         )
    //     })?;

    //     Ok(CallToolResult::success(vec![content]))
    // }

    // Balance Change endpoint
    #[tool(description = "Get detailed balance change activities")]
    async fn balance_change(
        &self,
        #[tool(aggr)] request: BalanceChangeRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "account": request.account,
        });

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(before_tx) = request.before_tx {
            params["before_tx"] = json!(before_tx);
        }

        let response = self
            .make_request("/account/balance_change", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Transaction Detail endpoint
    #[tool(description = "Get detailed transaction information")]
    async fn transaction_detail(
        &self,
        #[tool(aggr)] request: TransactionDetailRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "tx": request.tx,
        });

        let response = self
            .make_request("/transaction/detail", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Transaction Actions endpoint
    #[tool(description = "Get parsed actions from a transaction")]
    async fn transaction_actions(
        &self,
        #[tool(aggr)] request: TransactionActionsRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "tx": request.tx,
        });

        let response = self
            .make_request("/transaction/actions", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }
}

#[tool(tool_box)]
impl ServerHandler for SolscanApi {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides tools to access the Solscan API for Solana blockchain data. Use these tools to fetch token information, account activities, and transaction details on the Solana blockchain.".to_string()),
        }
    }
}
