use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static BALANCE_KEY: &[u8] = b"balances";
pub static EXCHANGE_KEY: &[u8] = b"exchange";
pub static BORROWS_KEY: &[u8] = b"borrows";
pub static COLLATERAL_KEY: &[u8] = b"collateral";

/// Config struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// name in config
    pub name: String,
    pub total_supply: Uint128,
    pub decimals: Uint128,
    pub symbol: String,
    pub intital_exchange_rate: f64,
}

/// Config singleton initialization
pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Config> {
    singleton(storage, CONFIG_KEY)
}

/// Get config
pub fn get_config<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY).load()
}

/// Set config
pub fn set_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_KEY).save(config)
}

/// Get balance from address
pub fn get_balance<S: Storage>(storage: &S, addr: CanonicalAddr) -> Option<CanonicalAddr> {
    match ReadonlyBucket::new(BALANCE_KEY, storage).may_load(&addr.as_slice()) {
        Ok(Some(wrapped_address)) => wrapped_address,
        _ => None,
    }
}

/// Set name from address
pub fn set_balance<S: Storage>(
    storage: &mut S,
    address: CanonicalAddr,
    amount: Uint128,
) -> StdResult<()> {
    match Bucket::new(BALANCE_KEY, storage).save(&address.as_slice(), &amount) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            address, amount
        ))),
    }
}

/// Get exchange rate
pub fn get_exchange_rate<S: Storage>(storage: &mut S) -> StdResult<Uint128> {
    ReadonlySingleton::new(storage, EXCHANGE_KEY).load()
}

/// Set exchange rate
pub fn set_exchange_rate<S: Storage>(storage: &mut S, rate: f64) -> StdResult<()> {
    Singleton::new(storage, EXCHANGE_KEY).save(&rate)
}

/// Get exchange rate
pub fn get_cash<S: Storage>(storage: &mut S) -> StdResult<Uint128> {
    ReadonlySingleton::new(storage, COLLATERAL_KEY).load()
}

/// Set exchange rate
pub fn set_cash<S: Storage>(storage: &mut S, amount: Uint128) -> StdResult<()> {
    Singleton::new(storage, COLLATERAL_KEY).save(&amount)
}
