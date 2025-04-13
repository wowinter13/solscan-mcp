# 🟣🔎 Solscan API Tools

## Features

This MCP server exposes the following tools for interacting with the Solscan API:

1. `chain_info`
   - Get Solana blockchain information
   - Input:
     - No parameters required
   - Returns: Blockchain information including block height, current epoch, absolute slot, and transaction count

2. `token_meta`
   - Get token metadata
   - Input:
     - `token_address` (string): A token address on Solana blockchain
   - Returns: Token metadata including name, symbol, price, market cap, etc.

3. `token_meta_multi`
   - Get the metadata of multiple tokens (max 20 tokens)
   - Input:
     - `address` (string[]): Array of token addresses (max 20)
   - Returns: Metadata for multiple tokens

4. `token_markets`
   - Get token market data and liquidity pools
   - Inputs:
     - `token_address` (string): Token address to query
     - `sort_by` (string, optional): Field to sort by
     - `program` (string[], optional): Filter by program addresses (max 5)
     - `page` (number, optional): Page number (default: 1)
     - `page_size` (number, optional): Items per page (10, 20, 30, 40, 60, 100)
   - Returns: Market data and liquidity pools for the token

5. `token_holders`
   - Get token holder distribution
   - Inputs:
     - `token_address` (string): Token address to query
     - `page` (number, optional): Page number (default: 1)
     - `page_size` (number, optional): Items per page (10, 20, 30, 40)
     - `from_amount` (string, optional): Minimum token holding amount
     - `to_amount` (string, optional): Maximum token holding amount
   - Returns: List of token holders with their balances

6. `token_list`
   - Get the list of tokens
   - Inputs:
     - `sort_by` (string, optional): Field to sort by
     - `sort_order` (string, optional): Sort order (asc/desc)
     - `page` (number, optional): Page number
     - `page_size` (number, optional): Items per page
   - Returns: List of tokens with their information

7. `token_top`
   - Get the list of top tokens
   - Input:
     - No parameters required
   - Returns: List of top tokens on Solana

8. `token_trending`
   - Get the list of trending tokens
   - Input:
     - `limit` (number, optional): Number of tokens to return
   - Returns: List of trending tokens

9. `token_price`
   - Get historical token price data
   - Inputs:
     - `token_address` (string): Token address to query
     - `from_time` (number, optional): Start date in UNIX timestamp format
     - `to_time` (number, optional): End date in UNIX timestamp format
   - Returns: Historical price data for the specified date range

10. `token_price_multi`
    - Get historical price data for multiple tokens
    - Inputs:
      - `address` (string[]): Array of token addresses (max 20)
      - `from_time` (number, optional): Start date in UNIX timestamp
      - `to_time` (number, optional): End date in UNIX timestamp
    - Returns: Historical price data for multiple tokens

11. `token_transfer`
    - Get transfer data of a token
    - Inputs:
      - `address` (string): Token address
      - `activity_type` (string, optional): Type of activity
      - `from` (string, optional): Sender address
      - `to` (string, optional): Recipient address
      - `amount` (string, optional): Transfer amount
      - `block_time` (string, optional): Block time
      - `exclude_amount_zero` (boolean, optional): Exclude zero amount transfers
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `sort_by` (string, optional): Field to sort by
      - `sort_order` (string, optional): Sort order
      - `value` (string, optional): Value filter
    - Returns: List of token transfers

12. `token_defi_activities`
    - Get defi activities involving a token
    - Inputs:
      - `address` (string): Token address
      - `from` (string, optional): From address
      - `platform` (string, optional): DeFi platform
      - `source` (string, optional): Source
      - `activity_type` (string, optional): Type of activity
      - `token` (string, optional): Related token address
      - `from_time` (number, optional): Start time
      - `to_time` (number, optional): End time
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `sort_by` (string, optional): Field to sort by
      - `sort_order` (string, optional): Sort order
    - Returns: List of DeFi activities for the token

13. `account_transfer`
    - Get transfer data of an account
    - Inputs:
      - `address` (string): Account address
      - `activity_type` (string, optional): Type of activity
      - `token_account` (string, optional): Token account address
      - `from` (string, optional): Sender address
      - `to` (string, optional): Recipient address
      - `token` (string, optional): Token address
      - `amount` (string, optional): Transfer amount
      - `from_time` (number, optional): Start time
      - `to_time` (number, optional): End time
      - `exclude_amount_zero` (boolean, optional): Exclude zero amount transfers
      - `flow` (string, optional): Direction of flow
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `sort_by` (string, optional): Field to sort by
      - `sort_order` (string, optional): Sort order
      - `value` (string, optional): Value filter
    - Returns: List of account transfers

14. `account_detail`
    - Get the details of an account
    - Input:
      - `address` (string): Account address
    - Returns: Detailed information about the account

15. `balance_change`
    - Get detailed balance change activities
    - Inputs:
      - `account` (string): Wallet address to query
      - `token_account` (string, optional): Token account address
      - `token` (string, optional): Token address
      - `from_time` (number, optional): Start time
      - `to_time` (number, optional): End time
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `remove_spam` (boolean, optional): Remove spam tokens
      - `amount` (string, optional): Amount filter
      - `flow` (string, optional): Direction of flow
      - `sort_by` (string, optional): Field to sort by
      - `sort_order` (string, optional): Sort order
      - `before_tx` (string, optional): Transaction signature to paginate from
    - Returns: List of balance changes for the specified wallet

16. `transaction_detail`
    - Get detailed transaction information
    - Input:
      - `tx` (string): Transaction signature
    - Returns: Detailed information about the transaction

17. `transaction_last`
    - Get the list of the latest transactions
    - Inputs:
      - `limit` (number, optional): Number of transactions to return
      - `filter` (string, optional): Filter criteria
    - Returns: List of the latest transactions

18. `transaction_actions`
    - Get parsed actions from a transaction
    - Input:
      - `tx` (string): Transaction signature
    - Returns: Parsed actions from the transaction

19. `block_last`
    - Get the list of the latest blocks
    - Input:
      - `limit` (number, optional): Number of blocks to return
    - Returns: List of the latest blocks

20. `block_transactions`
    - Get the list of transactions of a block
    - Inputs:
      - `block` (number/string): Block number or hash
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `exclude_vote` (boolean, optional): Exclude vote transactions
      - `program` (string, optional): Filter by program address
    - Returns: List of transactions in the block

21. `block_detail`
    - Get the details of a block
    - Input:
      - `block` (number/string): Block number or hash
    - Returns: Detailed information about the block

22. `market_list`
    - Get the list of pool markets
    - Inputs:
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `program` (string, optional): Filter by program address
    - Returns: List of pool markets

23. `market_info`
    - Get token market info
    - Input:
      - `address` (string): Market address
    - Returns: Detailed market information

24. `market_volume`
    - Get token market volume
    - Inputs:
      - `address` (string): Market address
      - `time` (string, optional): Time period
    - Returns: Market volume data

25. `account_defi_activities`
    - Get defi activities involving an account
    - Inputs:
      - `address` (string): Account address
      - `activity_type` (string, optional): Type of activity
      - `from` (string, optional): From address
      - `platform` (string, optional): DeFi platform
      - `source` (string, optional): Source
      - `token` (string, optional): Token address
      - `from_time` (number, optional): Start time
      - `to_time` (number, optional): End time
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `sort_by` (string, optional): Field to sort by
      - `sort_order` (string, optional): Sort order
    - Returns: List of DeFi activities for the account

26. `account_transactions`
    - Get the list of transactions of an account
    - Inputs:
      - `address` (string): Account address
      - `before` (string, optional): Transaction signature to paginate from
      - `limit` (number, optional): Number of transactions to return
    - Returns: List of account transactions

27. `account_portfolio`
    - Get the portfolio for a given address
    - Input:
      - `address` (string): Account address
    - Returns: Portfolio information including tokens and their values

28. `account_token_accounts`
    - Get token accounts of an account
    - Inputs:
      - `address` (string): Account address
      - `type` (string): Account type
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `hide_zero` (boolean, optional): Hide zero balance accounts
    - Returns: List of token accounts

29. `account_stake`
    - Get the list of stake accounts of an account
    - Inputs:
      - `address` (string): Account address
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
    - Returns: List of stake accounts

30. `account_reward_export`
    - Export the rewards for an account
    - Inputs:
      - `address` (string): Account address
      - `time_from` (number, optional): Start time
      - `time_to` (number, optional): End time
    - Returns: Rewards data in exportable format

31. `account_transfer_export`
    - Export transfer data of an account
    - Inputs:
      - `address` (string): Account address
      - `activity_type` (string, optional): Type of activity
      - `token_account` (string, optional): Token account address
      - `from` (string, optional): Sender address
      - `to` (string, optional): Recipient address
      - `token` (string, optional): Token address
      - `amount` (string, optional): Transfer amount
      - `from_time` (number, optional): Start time
      - `to_time` (number, optional): End time
      - `exclude_amount_zero` (boolean, optional): Exclude zero amount transfers
      - `flow` (string, optional): Direction of flow
    - Returns: Transfer data in exportable format

32. `account_metadata`
    - Get the metadata of an account
    - Input:
      - `address` (string): Account address
    - Returns: Account metadata information

33. `nft_news`
    - Get the list of new NFTs
    - Inputs:
      - `filter` (string): Filter criteria
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
    - Returns: List of new NFTs

34. `nft_activities`
    - Get NFT activities
    - Inputs:
      - `from` (string, optional): From address
      - `to` (string, optional): To address
      - `source` (string, optional): Source
      - `activity_type` (string, optional): Type of activity
      - `from_time` (number, optional): Start time
      - `to_time` (number, optional): End time
      - `token` (string, optional): NFT token address
      - `collection` (string, optional): NFT collection
      - `currency_token` (string, optional): Currency token
      - `price` (string, optional): Price filter
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
    - Returns: List of NFT activities

35. `nft_collection_lists`
    - Get the list of NFT collections
    - Inputs:
      - `range` (number, optional): Time range
      - `sort_order` (string, optional): Sort order
      - `sort_by` (string, optional): Field to sort by
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
      - `collection` (string, optional): Collection name search
    - Returns: List of NFT collections

36. `nft_collection_items`
    - Get the list of items of an NFT collection
    - Inputs:
      - `collection` (string): Collection address
      - `sort_by` (string, optional): Field to sort by
      - `page` (number, optional): Page number
      - `page_size` (number, optional): Items per page
    - Returns: List of NFT items in the collection 