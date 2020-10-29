use cosmwasm_std::{
    log, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    ReadonlyStorage, StdError, StdResult, Storage, Uint128,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use std::convert::TryInto;

use crate::state::{ALLOWANCE_PREFIX, BALANCE_PREFIX, get_state, set_state};

use crate::handler::interest_rates::*;

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
    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "mint"),
            log("sender", env.message.sender.as_str()),
            log("minted_amount", Uint128::from(0))
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

fn accrue_interest<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>) -> StdResult<HandleResponse> {
    let prior_state = get_state(&deps.storage);

    let borrow_rate = get_borrow_rate(prior_state.cash, prior_state.total_borrows, prior_state.total_reserves);
}