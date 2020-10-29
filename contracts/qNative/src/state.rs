use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_PREFIX: &[u8] = b"config";
pub static BALANCE_PREFIX: &[u8] = b"balances";
pub static ALLOWANCE_PREFIX: &[u8] = b"allowance";
pub static STATE_PREFIX: &[u8] = b"state";

/// Config struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,
    pub total_supply: Uint128,
    pub decimals: u8,
    pub symbol: String,
    pub intital_exchange_rate: f64,
}

pub struct State {
    pub underlying_asset: Uint128,
    pub total_reserves: Uint128,
    pub total_borrows: Uint128,
    pub interest_rate: f64,
    pub exchange_rate: f64,
    pub reserve_factor: f64,
}

/// Config singleton initialization
pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Config> {
    singleton(storage, CONFIG_PREFIX)
}

/// Get config
pub fn get_config<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_PREFIX).load()
}

/// Set config
pub fn set_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_PREFIX).save(config)
}

/// Get exchange rate
pub fn get_state<S: Storage>(storage: &mut S) -> StdResult<Uint128> {
    ReadonlySingleton::new(storage, STATE_PREFIX).load()
}

/// Set exchange rate
pub fn set_state<S: Storage>(storage: &mut S, rate: f64) -> StdResult<()> {
    Singleton::new(storage, STATE_PREFIX).save(&rate)
}
