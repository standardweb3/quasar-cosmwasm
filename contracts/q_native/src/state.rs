use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage, Uint128, ReadonlyStorage};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton, ReadonlyPrefixedStorage, PrefixedStorage};
use std::convert::TryInto;

pub static CONFIG_PREFIX: &[u8] = b"config";
pub static BALANCE_PREFIX: &[u8] = b"balances";
pub static ALLOWANCE_PREFIX: &[u8] = b"allowance";
pub static STATE_PREFIX: &[u8] = b"state";

/// Config struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,
    pub total_supply: u128,
    pub decimals: u8,
    pub symbol: String,
    pub initial_exchange_rate: u128,
    pub reserve_factor: u128
}

/// State struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub cash: u128,
    pub block_number: u64,
    pub total_reserves: u128,
    pub total_borrows: u128,
    pub interest_rate: u128,
    pub exchange_rate: u128,
    pub reserve_factor: u128,
    pub max_borrow_rate: u128,
    pub borrow_index: u128
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
pub fn get_state<S: Storage>(storage: &S) -> StdResult<State> {
    ReadonlySingleton::new(storage, STATE_PREFIX).load()
}

/// Set exchange rate
pub fn set_state<S: Storage>(storage: &mut S, state: &State) -> StdResult<()> {
    Singleton::new(storage, STATE_PREFIX).save(state)
}

/// Get balance from address
pub fn get_balance<S: Storage>(store: &S, owner: &CanonicalAddr) -> StdResult<u128> {
    let balance_store = ReadonlyPrefixedStorage::new(BALANCE_PREFIX, store);
    to_u128(&balance_store, owner.as_slice())
}

// Reads 16 byte storage value into u128
// Returns zero if key does not exist. Errors if data found that is not 16 bytes
pub fn to_u128<S: ReadonlyStorage>(store: &S, key: &[u8]) -> StdResult<u128> {
    let result = store.get(key);
    match result {
        Some(data) => bytes_to_u128(&data),
        None => Ok(0u128),
    }
}

// Converts 16 bytes value into u128
// Errors if data found that is not 16 bytes
pub fn bytes_to_u128(data: &[u8]) -> StdResult<u128> {
    match data[0..16].try_into() {
        Ok(bytes) => Ok(u128::from_be_bytes(bytes)),
        Err(_) => Err(StdError::generic_err(
            "Corrupted data found. 16 byte expected.",
        )),
    }
}

/// Get allowance from address
pub fn get_allowance<S: Storage>(
    store: &S,
    owner: &CanonicalAddr,
    spender: &CanonicalAddr,
) -> StdResult<u128> {
    let allowances_store = ReadonlyPrefixedStorage::new(ALLOWANCE_PREFIX, store);
    let owner_store = ReadonlyPrefixedStorage::new(owner.as_slice(), &allowances_store);
    to_u128(&owner_store, spender.as_slice())
}

/// Set allowance from address
pub fn set_allowance<S: Storage>(
    store: &mut S,
    owner: &CanonicalAddr,
    spender: &CanonicalAddr,
    amount: u128,
) -> StdResult<()> {
    let mut allowances_store = PrefixedStorage::new(ALLOWANCE_PREFIX, store);
    let mut owner_store = PrefixedStorage::new(owner.as_slice(), &mut allowances_store);
    owner_store.set(spender.as_slice(), &amount.to_be_bytes());
    Ok(())
}
