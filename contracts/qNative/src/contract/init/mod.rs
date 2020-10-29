use cosmwasm_std::{Api, Env, Extern, InitResponse, Querier, StdResult, Storage};

use crate::msg::InitMsg;
use crate::state::{config, Config};

/// Contract instantiation tx
/// tx inputs are specified in InitMsg in msg.rs file
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = Config {
        name: msg.name,
        total_supply: msg.total_supply,
        decimals: msg.decimals,
        symbol: msg.symbol,
        intital_exchange_rate: msg.intital_exchange_rate,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}
