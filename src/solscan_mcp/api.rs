use std::sync::Arc;

use rmcp::{model::*, schemars, tool, Error as McpError, ServerHandler};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::solscan_mcp::requests::account::*;
use crate::solscan_mcp::requests::block::*;
use crate::solscan_mcp::requests::market::*;
use crate::solscan_mcp::requests::nft::*;
use crate::solscan_mcp::requests::token::*;
use crate::solscan_mcp::requests::transaction::*;

// Base URLs for Solscan API
const SOLSCAN_API_BASE_URL: &str = "https://pro-api.solscan.io/v2.0";
const SOLSCAN_PUBLIC_API_BASE_URL: &str = "https://public-api.solscan.io";
// const WSOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ChainInfoRequest {
    // No parameters
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

    // Token Meta Multi endpoint
    #[tool(description = "Get the metadata of multiple tokens (max 20 tokens)")]
    async fn token_meta_multi(
        &self,
        #[tool(aggr)] request: TokenMetaMultiRequest,
    ) -> Result<CallToolResult, McpError> {
        // Check if the number of addresses doesn't exceed the maximum (20)
        if request.address.len() > 20 {
            return Err(McpError::internal_error(
                "Maximum number of token addresses (20) exceeded",
                None,
            ));
        }

        let params = json!({
            "address": request.address,
        });

        let response = self.make_request("/token/meta/multi", Some(params)).await?;

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

    // Token List endpoint
    #[tool(description = "Get the list of tokens")]
    async fn token_list(
        &self,
        #[tool(aggr)] request: TokenListRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional parameters
        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        let response = self.make_request("/token/list", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Top endpoint
    #[tool(description = "Get the list of top tokens")]
    async fn token_top(
        &self,
        #[tool(aggr)] _request: TokenTopRequest,
    ) -> Result<CallToolResult, McpError> {
        // This endpoint doesn't require any parameters
        let response = self.make_request("/token/top", None).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Trending endpoint
    #[tool(description = "Get the list of trending tokens")]
    async fn token_trending(
        &self,
        #[tool(aggr)] request: TokenTrendingRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional limit parameter
        if let Some(limit) = request.limit {
            params["limit"] = json!(limit);
        }

        let response = self.make_request("/token/trending", Some(params)).await?;

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

    // Token Price Multi endpoint
    #[tool(description = "Get historical price data for multiple tokens")]
    async fn token_price_multi(
        &self,
        #[tool(aggr)] request: TokenPriceMultiRequest,
    ) -> Result<CallToolResult, McpError> {
        // Check if the number of addresses doesn't exceed a reasonable limit (using 20 as in meta/multi)
        if request.address.len() > 20 {
            return Err(McpError::internal_error(
                "Maximum number of token addresses (20) exceeded",
                None,
            ));
        }

        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        let response = self
            .make_request("/token/price/multi", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Transfer endpoint
    #[tool(description = "Get transfer data of a token")]
    async fn token_transfer(
        &self,
        #[tool(aggr)] request: TokenTransferRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(activity_type) = &request.activity_type {
            params["activity_type"] = json!(activity_type);
        }

        if let Some(from) = &request.from {
            params["from"] = json!(from);
        }

        if let Some(to) = &request.to {
            params["to"] = json!(to);
        }

        if let Some(amount) = &request.amount {
            params["amount"] = json!(amount);
        }

        if let Some(block_time) = &request.block_time {
            params["block_time"] = json!(block_time);
        }

        if let Some(exclude_amount_zero) = request.exclude_amount_zero {
            params["exclude_amount_zero"] = json!(exclude_amount_zero);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        if let Some(value) = &request.value {
            params["value"] = json!(value);
        }

        let response = self.make_request("/token/transfer", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Token Defi Activities endpoint
    #[tool(description = "Get defi activities involving a token")]
    async fn token_defi_activities(
        &self,
        #[tool(aggr)] request: TokenDefiActivitiesRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(from) = &request.from {
            params["from"] = json!(from);
        }

        if let Some(platform) = &request.platform {
            params["platform"] = json!(platform);
        }

        if let Some(source) = &request.source {
            params["source"] = json!(source);
        }

        if let Some(activity_type) = &request.activity_type {
            params["activity_type"] = json!(activity_type);
        }

        if let Some(token) = &request.token {
            params["token"] = json!(token);
        }

        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        let response = self
            .make_request("/token/defi/activities", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Transfer endpoint
    #[tool(description = "Get transfer data of an account")]
    async fn account_transfer(
        &self,
        #[tool(aggr)] request: AccountTransferRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(activity_type) = &request.activity_type {
            params["activity_type"] = json!(activity_type);
        }

        if let Some(token_account) = &request.token_account {
            params["token_account"] = json!(token_account);
        }

        if let Some(from) = &request.from {
            params["from"] = json!(from);
        }

        if let Some(to) = &request.to {
            params["to"] = json!(to);
        }

        if let Some(token) = &request.token {
            params["token"] = json!(token);
        }

        if let Some(amount) = &request.amount {
            params["amount"] = json!(amount);
        }

        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        if let Some(exclude_amount_zero) = request.exclude_amount_zero {
            params["exclude_amount_zero"] = json!(exclude_amount_zero);
        }

        if let Some(flow) = &request.flow {
            params["flow"] = json!(flow);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        if let Some(value) = &request.value {
            params["value"] = json!(value);
        }

        let response = self.make_request("/account/transfer", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Detail endpoint
    #[tool(description = "Get the details of an account")]
    async fn account_detail(
        &self,
        #[tool(aggr)] request: AccountDetailRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "address": request.address,
        });

        let response = self.make_request("/account/detail", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Balance Change endpoint
    #[tool(description = "Get detailed balance change activities")]
    async fn balance_change(
        &self,
        #[tool(aggr)] request: BalanceChangeRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.account,
        });

        // Add optional parameters
        if let Some(token_account) = &request.token_account {
            params["token_account"] = json!(token_account);
        }

        if let Some(token) = &request.token {
            params["token"] = json!(token);
        }

        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(remove_spam) = &request.remove_spam {
            params["remove_spam"] = json!(remove_spam);
        }

        if let Some(amount) = &request.amount {
            params["amount"] = json!(amount);
        }

        if let Some(flow) = &request.flow {
            params["flow"] = json!(flow);
        }

        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        if let Some(before_tx) = &request.before_tx {
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

    // Transaction Last endpoint
    #[tool(description = "Get the list of the latest transactions")]
    async fn transaction_last(
        &self,
        #[tool(aggr)] request: TransactionLastRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional parameters
        if let Some(limit) = request.limit {
            params["limit"] = json!(limit);
        }

        if let Some(filter) = &request.filter {
            params["filter"] = json!(filter);
        }

        let response = self.make_request("/transaction/last", Some(params)).await?;

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

    // Block Last endpoint
    #[tool(description = "Get the list of the latest blocks")]
    async fn block_last(
        &self,
        #[tool(aggr)] request: BlockLastRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional limit parameter
        if let Some(limit) = request.limit {
            params["limit"] = json!(limit);
        }

        let response = self.make_request("/block/last", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Block Transactions endpoint
    #[tool(description = "Get the list of transactions of a block")]
    async fn block_transactions(
        &self,
        #[tool(aggr)] request: BlockTransactionsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "block": request.block,
        });

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(exclude_vote) = request.exclude_vote {
            params["exclude_vote"] = json!(exclude_vote);
        }

        if let Some(program) = &request.program {
            params["program"] = json!(program);
        }

        let response = self
            .make_request("/block/transactions", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Block Detail endpoint
    #[tool(description = "Get the details of a block")]
    async fn block_detail(
        &self,
        #[tool(aggr)] request: BlockDetailRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "block": request.block,
        });

        let response = self.make_request("/block/detail", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Market List endpoint
    #[tool(description = "Get the list of pool markets")]
    async fn market_list(
        &self,
        #[tool(aggr)] request: MarketListRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(program) = &request.program {
            params["program"] = json!(program);
        }

        let response = self.make_request("/market/list", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Market Info endpoint
    #[tool(description = "Get token market info")]
    async fn market_info(
        &self,
        #[tool(aggr)] request: MarketInfoRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "address": request.address,
        });

        let response = self.make_request("/market/info", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Market Volume endpoint
    #[tool(description = "Get token market volume")]
    async fn market_volume(
        &self,
        #[tool(aggr)] request: MarketVolumeRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional time parameter
        if let Some(time) = &request.time {
            params["time"] = json!(time);
        }

        let response = self.make_request("/market/volume", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Defi Activities endpoint
    #[tool(description = "Get defi activities involving an account")]
    async fn account_defi_activities(
        &self,
        #[tool(aggr)] request: AccountDefiActivitiesRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(activity_type) = &request.activity_type {
            params["activity_type"] = json!(activity_type);
        }

        if let Some(from) = &request.from {
            params["from"] = json!(from);
        }

        if let Some(platform) = &request.platform {
            params["platform"] = json!(platform);
        }

        if let Some(source) = &request.source {
            params["source"] = json!(source);
        }

        if let Some(token) = &request.token {
            params["token"] = json!(token);
        }

        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        let response = self
            .make_request("/account/defi/activities", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Transactions endpoint
    #[tool(description = "Get the list of transactions of an account")]
    async fn account_transactions(
        &self,
        #[tool(aggr)] request: AccountTransactionsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(before) = &request.before {
            params["before"] = json!(before);
        }

        if let Some(limit) = request.limit {
            params["limit"] = json!(limit);
        }

        let response = self
            .make_request("/account/transactions", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Portfolio endpoint
    #[tool(description = "Get the portfolio for a given address")]
    async fn account_portfolio(
        &self,
        #[tool(aggr)] request: AccountPortfolioRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "address": request.address,
        });

        let response = self
            .make_request("/account/portfolio", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Token-Accounts endpoint
    #[tool(description = "Get token accounts of an account")]
    async fn account_token_accounts(
        &self,
        #[tool(aggr)] request: AccountTokenAccountsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
            "type": request.r#type,
        });

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(hide_zero) = request.hide_zero {
            params["hide_zero"] = json!(hide_zero);
        }

        let response = self
            .make_request("/account/token-accounts", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Stake endpoint
    #[tool(description = "Get the list of stake accounts of an account")]
    async fn account_stake(
        &self,
        #[tool(aggr)] request: AccountStakeRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        let response = self.make_request("/account/stake", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Stake Rewards Export endpoint
    #[tool(description = "Export the rewards for an account")]
    async fn account_reward_export(
        &self,
        #[tool(aggr)] request: AccountRewardExportRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(time_from) = request.time_from {
            params["time_from"] = json!(time_from);
        }

        if let Some(time_to) = request.time_to {
            params["time_to"] = json!(time_to);
        }

        let response = self
            .make_request("/account/reward/export", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Transfer Export endpoint
    #[tool(description = "Export transfer data of an account")]
    async fn account_transfer_export(
        &self,
        #[tool(aggr)] request: AccountTransferExportRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "address": request.address,
        });

        // Add optional parameters
        if let Some(activity_type) = &request.activity_type {
            params["activity_type"] = json!(activity_type);
        }

        if let Some(token_account) = &request.token_account {
            params["token_account"] = json!(token_account);
        }

        if let Some(from) = &request.from {
            params["from"] = json!(from);
        }

        if let Some(to) = &request.to {
            params["to"] = json!(to);
        }

        if let Some(token) = &request.token {
            params["token"] = json!(token);
        }

        if let Some(amount) = &request.amount {
            params["amount"] = json!(amount);
        }

        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        if let Some(exclude_amount_zero) = request.exclude_amount_zero {
            params["exclude_amount_zero"] = json!(exclude_amount_zero);
        }

        if let Some(flow) = &request.flow {
            params["flow"] = json!(flow);
        }

        let response = self
            .make_request("/account/transfer/export", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // Account Metadata endpoint
    #[tool(description = "Get the metadata of an account")]
    async fn account_metadata(
        &self,
        #[tool(aggr)] request: AccountMetadataRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = json!({
            "address": request.address,
        });

        let response = self.make_request("/account/metadata", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // NFT News endpoint
    #[tool(description = "Get the list of new NFTs")]
    async fn nft_news(
        &self,
        #[tool(aggr)] request: NftNewsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "filter": request.filter,
        });

        // Add optional parameters
        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        let response = self.make_request("/nft/news", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // NFT Activities endpoint
    #[tool(description = "Get NFT activities")]
    async fn nft_activities(
        &self,
        #[tool(aggr)] request: NftActivitiesRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional parameters
        if let Some(from) = &request.from {
            params["from"] = json!(from);
        }

        if let Some(to) = &request.to {
            params["to"] = json!(to);
        }

        if let Some(source) = &request.source {
            params["source"] = json!(source);
        }

        if let Some(activity_type) = &request.activity_type {
            params["activity_type"] = json!(activity_type);
        }

        if let Some(from_time) = request.from_time {
            params["from_time"] = json!(from_time);
        }

        if let Some(to_time) = request.to_time {
            params["to_time"] = json!(to_time);
        }

        if let Some(token) = &request.token {
            params["token"] = json!(token);
        }

        if let Some(collection) = &request.collection {
            params["collection"] = json!(collection);
        }

        if let Some(currency_token) = &request.currency_token {
            params["currency_token"] = json!(currency_token);
        }

        if let Some(price) = &request.price {
            params["price"] = json!(price);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        let response = self.make_request("/nft/activities", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // NFT Collection Lists endpoint
    #[tool(description = "Get the list of NFT collections")]
    async fn nft_collection_lists(
        &self,
        #[tool(aggr)] request: NftCollectionListsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({});

        // Add optional parameters
        if let Some(range) = request.range {
            params["range"] = json!(range);
        }

        if let Some(sort_order) = &request.sort_order {
            params["sort_order"] = json!(sort_order);
        }

        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        if let Some(collection) = &request.collection {
            params["collection"] = json!(collection);
        }

        let response = self
            .make_request("/nft/collection/lists", Some(params))
            .await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }

    // NFT Collection Items endpoint
    #[tool(description = "Get the list of items of a NFT collection")]
    async fn nft_collection_items(
        &self,
        #[tool(aggr)] request: NftCollectionItemsRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut params = json!({
            "collection": request.collection,
        });

        // Add optional parameters
        if let Some(sort_by) = &request.sort_by {
            params["sort_by"] = json!(sort_by);
        }

        if let Some(page) = request.page {
            params["page"] = json!(page);
        }

        if let Some(page_size) = request.page_size {
            params["page_size"] = json!(page_size);
        }

        let response = self
            .make_request("/nft/collection/items", Some(params))
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
