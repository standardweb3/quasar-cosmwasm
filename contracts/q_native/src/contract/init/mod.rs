use cosmwasm_std::{Api, Env, Extern, InitResponse, Querier, StdResult, Storage, Uint128};

use crate::msg::InitMsg;
use crate::state::{config, Config, set_state, State};

/// Contract instantiation tx
/// tx inputs are specified in InitMsg in msg.rs file
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let init_config = Config {
        name: msg.name,
        total_supply: msg.total_supply,
        decimals: msg.decimals,
        symbol: msg.symbol,
        denom: msg.denom,
        initial_exchange_rate: msg.initial_exchange_rate,
        reserve_factor: msg.reserve_factor,
        max_borrow_rate: msg.max_borrow_rate,
        borrow_index: msg.borrow_index,
    };

    config(&mut deps.storage).save(&init_config)?;

    
    let init_state = State {
        cash: Uint128::from(0u128),
        block_number: env.block.height,
        total_reserves: Uint128::from(0u128),
        total_borrows: Uint128::from(0u128),
        exchange_rate: init_config.initial_exchange_rate,
        reserve_factor: init_config.reserve_factor,
        max_borrow_rate: init_config.max_borrow_rate,//100_000_000_000,
        borrow_index: init_config.borrow_index
    };

    set_state(&mut deps.storage, &init_state)?;
    

    Ok(InitResponse::default())
}
