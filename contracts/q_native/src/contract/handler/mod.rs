use cosmwasm_std::{Api, Empty, Env, Extern, HandleResponse, Querier, StdResult, Storage};

use crate::msg::HandleMsg;

mod collateral;
mod token;
mod exponential;
mod interest_model;

/// General handler for contract tx input
/// tx inputs are defined HandleMsg enum in msg.rs file
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse<Empty>> {
    match msg {
        HandleMsg::Approve { spender, amount } => token::try_approve(deps, env, &spender, &amount),
        HandleMsg::Transfer { recipient, amount } => {
            token::try_transfer(deps, env, &recipient, &amount)
        }
        HandleMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => token::try_transfer_from(deps, env, &owner, &recipient, &amount),
        HandleMsg::Mint {} => collateral::try_mint(deps, env),
        HandleMsg::Redeem {redeem_tokens_in} => collateral::try_redeem(deps, env, redeem_tokens_in),
        HandleMsg::RepayBorrow {} => collateral::try_repay_borrow(deps, env),
        HandleMsg::Borrow{ borrow_amount } => collateral::try_borrow(deps, env, borrow_amount)
    }
}
