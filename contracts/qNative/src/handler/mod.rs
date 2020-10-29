use cosmwasm_std::{Api, Empty, Env, Extern, HandleResponse, Querier, StdResult, Storage};

use crate::msg::HandleMsg;

mod token;

/// General handler for contract tx input
/// tx inputs are defined HandleMsg enum in msg.rs file
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse<Empty>> {
    match msg {
        HandleMsg::Approve { spender, amount } => {
            token::try_approve(deps, env, info, &spender, &amount)
        }
        HandleMsg::Transfer { recipient, amount } => {
            token::try_transfer(deps, env, info, &recipient, &amount)
        }
        HandleMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => token::try_transfer_from(deps, env, info, &owner, &recipient, &amount),
        HandleMsg::Burn { amount } => token::try_burn(deps, env, info, &amount),
    }
}
