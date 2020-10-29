use cosmwasm_std::{
    log, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    ReadonlyStorage, StdError, StdResult, Storage, Uint128,
};

use std::convert::TryInto;

use crate::state::{get_state, set_state};

use crate::contract::handler::interest_model::{get_borrow_rate};
use crate::contract::handler::exponential::truncate;

pub fn try_repay_borrow<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "repay_borrow"),
            log("sender", env.message.sender.as_str()),
        ],
        data: None,
    };
    Ok(res)
}

pub fn try_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {

    accrue_interest(deps, env.clone())?;



    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "mint"),
            log("sender", env.message.sender.as_str()),
            log("minted_amount", 0)
        ],
        data: None,
    };
    Ok(res)
}

pub fn try_redeem<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "redeem"),
            log("sender", env.message.sender.as_str()),
        ],
        data: None,
    };
    Ok(res)
}

fn accrue_interest<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env) -> StdResult<HandleResponse> {
    let prior_state = get_state(&deps.storage)?;

    let borrow_rate = get_borrow_rate(&prior_state.cash, &prior_state.total_borrows, &prior_state.total_reserves);

    if borrow_rate > prior_state.max_borrow_rate {
        return Err(StdError::generic_err(format!(
            "borrow rate is absurdly high: borrow_rate: {}, max_borrow_rate: {}",
             borrow_rate, prior_state.max_borrow_rate)
            )
        );
    }

    let current_block = env.block.height;

    let block_delta: u128 = (prior_state.block_number - current_block).try_into().unwrap();

    // Calculate the interest accumulated into borrows and reserves and the new index:
    let simple_interest_factor = borrow_rate * block_delta;

    let accumulated_interest = truncate(simple_interest_factor * prior_state.total_borrows);
    let new_total_borrows = accumulated_interest + prior_state.total_borrows;
    let new_total_reserves = truncate(accumulated_interest * prior_state.reserve_factor) + prior_state.total_reserves;
    let new_borrow_index = truncate(simple_interest_factor * prior_state.borrow_index) + prior_state.borrow_index;

    // Setup new state
    let mut new_state = get_state(&deps.storage)?;
    new_state.block_number = env.block.height; 
    new_state.borrow_index = new_borrow_index;
    new_state.total_borrows = new_total_borrows;
    new_state.total_reserves = new_total_reserves;

    set_state(&mut deps.storage, &new_state);

    let mock_res = HandleResponse {
        messages: vec![],
        log: vec![
        ],
        data: None,
    };

    Ok(mock_res)
}