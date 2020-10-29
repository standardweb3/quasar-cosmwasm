use cosmwasm_std::{Api, Env, Extern, InitResponse, Querier, StdResult, Storage};

use crate::msg::InitMsg;
use crate::state::{config, Config, set_state};

/// Contract instantiation tx
/// tx inputs are specified in InitMsg in msg.rs file
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = Config {
        name: msg.name,
        total_supply: msg.total_supply.u128(),
        decimals: msg.decimals,
        symbol: msg.symbol,
        initial_exchange_rate: msg.intital_exchange_rate.u128(),
        reserve_factor: msg.reserve_factor.u128()
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}
