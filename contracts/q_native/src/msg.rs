use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub total_supply: Uint128,
    pub decimals: u8,
    pub symbol: String,
    pub initial_exchange_rate: Uint128,
    pub reserve_factor: Uint128,
    pub borrow_index: Uint128,
    pub max_borrow_rate: Uint128,
    pub denom: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Mint {},
    Redeem {
        redeem_tokens_in: Uint128
    },
    Borrow {
        borrow_amount: Uint128
    },
    RepayBorrow {},
    Approve {
        spender: HumanAddr,
        amount: Uint128,
    },
    Transfer {
        recipient: HumanAddr,
        amount: Uint128,
    },
    TransferFrom {
        owner: HumanAddr,
        recipient: HumanAddr,
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Balance {
        address: HumanAddr,
    },
    Allowance {
        owner: HumanAddr,
        spender: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub name: String,
    pub total_supply: Uint128,
    pub decimals: u8,
    pub symbol: String,
    pub intital_exchange_rate: Uint128,
    pub reserve_factor: Uint128,
    pub borrow_index: Uint128,
    pub denom: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct AllowanceResponse {
    pub allowance: Uint128,
}
