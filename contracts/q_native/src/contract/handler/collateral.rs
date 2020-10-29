use cosmwasm_std::{
    log, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    ReadonlyStorage, StdError, StdResult, Storage, Uint128,
};

use std::convert::TryInto;

use crate::state::{get_state, set_state, get_config, set_config};

use crate::contract::handler::interest_model::{get_borrow_rate};
use crate::contract::handler::exponential::truncate;
use crate::contract::handler::token::mint_tokens;

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

    let current_block = env.block.height;

    // Check native currency transfer
    let mint_amount = env.message.sent_funds[0].amount.u128(); 

    // Get exchange rate derived from borrow and reserve
    let exchange_rate = get_exchange_rate(deps, env.clone())?;

    let token_mint_amount = truncate(mint_amount / exchange_rate * 100_000_000);

    // Set new config
    let mut new_config = get_config(&deps.storage)?;
    new_config.total_supply += token_mint_amount;

    set_config(&mut deps.storage, &new_config);

    // Mint token to the sender
    let recipient_address_raw = deps.api.canonical_address(&env.message.sender)?;
    mint_tokens(
        &mut deps.storage,
        &recipient_address_raw,
        token_mint_amount,
    )?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "mint"),
            log("sender", env.message.sender.as_str()),
            log("minted_amount", token_mint_amount.clone())
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

fn accrue_interest<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env) -> StdResult<()>  {
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

    // Set new state
    let mut new_state = get_state(&deps.storage)?;
    new_state.block_number = env.block.height; 
    new_state.borrow_index = new_borrow_index;
    new_state.total_borrows = new_total_borrows;
    new_state.total_reserves = new_total_reserves;

    set_state(&mut deps.storage, &new_state);


    Ok(())
}

fn get_exchange_rate<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env) -> StdResult<u128> {
    let config = get_config(&deps.storage)?;
    
    // if total supply is zero
    if config.total_supply == 0 {
        return Ok(config.initial_exchange_rate);
    }
    // else calculate exchange rate
    let prior_state = get_state(&deps.storage)?;

    let total_cash = prior_state.cash;

    let cash_plus_borrows_minus_reserves = total_cash + prior_state.total_borrows - prior_state.total_reserves;

    let exchange_rate = cash_plus_borrows_minus_reserves * 100_000_000 / config.total_supply;


    Ok(exchange_rate)
}